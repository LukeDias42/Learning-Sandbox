#include "./constants.h"
#include <SDL2/SDL.h>
#include <SDL2/SDL_events.h>
#include <SDL2/SDL_keycode.h>
#include <SDL2/SDL_render.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>

typedef enum TileType {
  WALL,
  CLOSED_DOOR,
  OPEN_DOOR,
  EMPTY,
  OUT_OF_BOUNDS
} tile_type_t;

const tile_type_t map[MAP_NUM_ROWS][MAP_NUM_COLS] = {
    {WALL, WALL, WALL, EMPTY, WALL, WALL, WALL, WALL, WALL, WALL,
     WALL, WALL, WALL, WALL,  WALL, WALL, WALL, WALL, WALL, WALL},
    {WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
     EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL},
    {WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
     EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL},
    {WALL, WALL,  EMPTY, EMPTY, WALL, EMPTY, WALL, EMPTY, WALL, EMPTY,
     WALL, EMPTY, WALL,  EMPTY, WALL, EMPTY, WALL, EMPTY, WALL, WALL},
    {WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
     EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL},
    {WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL,
     EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL,  EMPTY, WALL},
    {WALL,  EMPTY, EMPTY, EMPTY, WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
     EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL,  EMPTY, WALL,  EMPTY, WALL},
    {WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,       WALL,  EMPTY,
     EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, CLOSED_DOOR, EMPTY, WALL},
    {WALL,  EMPTY, EMPTY, WALL, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
     EMPTY, EMPTY, EMPTY, WALL, WALL,  WALL,  WALL,  WALL,  EMPTY, WALL},
    {WALL,  EMPTY, WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
     EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL},
    {WALL, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
     WALL, WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL},
    {WALL,  EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
     EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, WALL},
    {WALL, WALL, WALL, WALL, WALL, WALL, WALL, WALL, WALL, WALL,
     WALL, WALL, WALL, WALL, WALL, WALL, WALL, WALL, WALL, WALL},
};

typedef struct Position {
  double_t x;
  double_t y;
} position_t;

typedef struct Player {
  uint8_t height;
  uint8_t width;
  uint8_t walkSpeed;
  int8_t turnDirection;
  int8_t walkDirection;
  double_t turnSpeed;
  position_t position;
  double_t rotationAngle;
} player_t;

typedef struct Ray {
  position_t wallHit;
  double_t rayAngle;
  double_t distance;
  tile_type_t wallHitType;
  uint8_t wasHitVertical;
  uint8_t isRayFacingUp;
  uint8_t isRayFacingRight;
} ray_t;

player_t player;
ray_t rays[NUM_RAYS];

SDL_Window *window = NULL;
SDL_Renderer *renderer = NULL;
int isGameRunning = FALSE;

int ticksLastFrame = 0;

double_t normalizeAngle(double_t angle) {
  if (angle > TWO_PI)
    return angle -= TWO_PI;
  if (angle < 0)
    return angle += TWO_PI;
  return angle;
}

double_t distanceBetweenPoints(double_t x1, double_t y1, double_t x2,
                               double_t y2) {
  return sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1));
}

int initializeWindow() {
  if (SDL_Init(SDL_INIT_EVERYTHING) != 0) {
    fprintf(stderr, "Error initializing SDL.\n ");
    return FALSE;
  }
  window = SDL_CreateWindow("Raycasting", SDL_WINDOWPOS_CENTERED,
                            SDL_WINDOWPOS_CENTERED, WINDOW_WIDTH, WINDOW_HEIGHT,
                            SDL_WINDOW_BORDERLESS);
  if (!window) {
    fprintf(stderr, "Error creating SDL window.\n");
    return FALSE;
  }

  renderer = SDL_CreateRenderer(window, -1, 0);
  if (!window) {
    fprintf(stderr, "Error creating SDL renderer.\n");
    return FALSE;
  }

  return TRUE;
}

void destroyWindow() {
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  SDL_Quit();
}

void setup() {
  player.position.x = WINDOW_WIDTH / 2.0f;
  player.position.y = WINDOW_HEIGHT / 2.0f;
  player.height = 1;
  player.width = 1;
  player.turnDirection = 0;
  player.walkDirection = 0;
  player.rotationAngle = PI / 2;
  player.walkSpeed = 100;
  player.turnSpeed = to_radian(90);
  ticksLastFrame = SDL_GetTicks();
}

tile_type_t mapTileAt(double_t x, double_t y) {
  if (x < 0 || x > WINDOW_WIDTH || y < 0 || y > WINDOW_HEIGHT) {
    return OUT_OF_BOUNDS;
  }
  uint8_t xIndex = floor(x / TILE_SIZE);
  uint8_t yIndex = floor(y / TILE_SIZE);
  return map[yIndex][xIndex];
}

void movePlayer(float deltaTime) {
  player.rotationAngle += player.turnSpeed * player.turnDirection * deltaTime;
  player.rotationAngle = normalizeAngle(player.rotationAngle);
  double_t newPlayerX =
      player.position.x + player.walkSpeed * player.walkDirection *
                              cos(player.rotationAngle) * deltaTime;
  double_t newPlayerY =
      player.position.y + player.walkSpeed * player.walkDirection *
                              sin(player.rotationAngle) * deltaTime;

  tile_type_t tileAtNewLocation = mapTileAt(newPlayerX, newPlayerY);
  if (tileAtNewLocation != WALL && tileAtNewLocation != CLOSED_DOOR &&
      tileAtNewLocation != OUT_OF_BOUNDS) {
    player.position.x = newPlayerX;
    player.position.y = newPlayerY;
  }
}

void renderPlayer() {
  SDL_SetRenderDrawColor(renderer, 0xff, 0x00, 0x00, 0xff);
  SDL_Rect playerRect = {MINIMAP_SCALE_FACTOR * player.position.x,
                         MINIMAP_SCALE_FACTOR * player.position.y,
                         MINIMAP_SCALE_FACTOR * player.width,
                         MINIMAP_SCALE_FACTOR * player.height};
  SDL_RenderFillRect(renderer, &playerRect);
}

typedef struct WallHit {
  uint8_t found;
  position_t hit;
  tile_type_t type;
} wall_hit_t;

wall_hit_t findHorizontalWallHit(uint32_t columnId, ray_t ray) {
  double_t xintercept, yintercept;
  double_t xstep, ystep;

  wall_hit_t wallHit = {FALSE, 0, 0, OUT_OF_BOUNDS};

  yintercept = floor(player.position.y / TILE_SIZE) * TILE_SIZE;
  yintercept += ray.isRayFacingUp ? 0 : TILE_SIZE;

  xintercept =
      player.position.x + (yintercept - player.position.y) / tan(ray.rayAngle);

  ystep = TILE_SIZE;
  ystep *= ray.isRayFacingUp ? -1 : 1;

  xstep = TILE_SIZE / tan(ray.rayAngle);
  xstep *= (ray.isRayFacingRight && xstep < 0) ? -1 : 1;
  xstep *= (!ray.isRayFacingRight && xstep > 0) ? -1 : 1;

  double_t nextTouchX = xintercept;
  double_t nextTouchY = yintercept;

  tile_type_t type;
  while (nextTouchX >= 0 && nextTouchX <= WINDOW_WIDTH && nextTouchY >= 0 &&
         nextTouchY <= WINDOW_HEIGHT && !wallHit.found) {
    type = mapTileAt(nextTouchX, nextTouchY + (ray.isRayFacingUp ? -1 : 0));
    switch (type) {
    case WALL:
    case CLOSED_DOOR:
    case OPEN_DOOR:
    case OUT_OF_BOUNDS:
      wallHit.found = TRUE;
      wallHit.hit.x = nextTouchX;
      wallHit.hit.y = nextTouchY;
      wallHit.type = type;
      break;
    case EMPTY:
      break;
    }
    nextTouchX += xstep;
    nextTouchY += ystep;
  }

  return wallHit;
}

wall_hit_t findVerticalWallHit(uint32_t columnId, ray_t ray) {
  double_t xintercept, yintercept;
  double_t xstep, ystep;

  wall_hit_t wallHit = {FALSE, 0, 0, OUT_OF_BOUNDS};

  xintercept = floor(player.position.x / TILE_SIZE) * TILE_SIZE;
  xintercept += ray.isRayFacingRight ? TILE_SIZE : 0;

  yintercept =
      player.position.y + (xintercept - player.position.x) * tan(ray.rayAngle);

  xstep = TILE_SIZE;
  xstep *= ray.isRayFacingRight ? 1 : -1;

  ystep = TILE_SIZE * tan(ray.rayAngle);
  ystep *= (ray.isRayFacingUp && ystep > 0) ? -1 : 1;
  ystep *= (!ray.isRayFacingUp && ystep < 0) ? -1 : 1;

  double_t nextTouchX = xintercept;
  double_t nextTouchY = yintercept;

  tile_type_t type;
  while (nextTouchX >= 0 && nextTouchX <= WINDOW_WIDTH && nextTouchY >= 0 &&
         nextTouchY <= WINDOW_HEIGHT && !wallHit.found) {
    type = mapTileAt(nextTouchX + (ray.isRayFacingRight ? 0 : -1), nextTouchY);
    switch (type) {
    case WALL:
    case CLOSED_DOOR:
    case OUT_OF_BOUNDS:
    case OPEN_DOOR:
      wallHit.found = TRUE;
      wallHit.hit.x = nextTouchX;
      wallHit.hit.y = nextTouchY;
      wallHit.type = type;
      break;
    case EMPTY:
      break;
    }
    nextTouchX += xstep;
    nextTouchY += ystep;
  }

  return wallHit;
}

void castRay(uint32_t columnId, double_t rayAngle) {
  ray_t ray = {.rayAngle = normalizeAngle(rayAngle),
               .isRayFacingUp = rayAngle > PI && rayAngle < TWO_PI,
               .isRayFacingRight =
                   rayAngle < PI * 0.5f || rayAngle > PI * 1.5f};

  wall_hit_t horzWallHit = findHorizontalWallHit(columnId, ray);
  wall_hit_t vertWallHit = findVerticalWallHit(columnId, ray);
  double_t horzDist =
      horzWallHit.found
          ? distanceBetweenPoints(player.position.x, player.position.y,
                                  horzWallHit.hit.x, horzWallHit.hit.y)
          : SDL_MAX_UINT32;
  double_t vertDist =
      vertWallHit.found
          ? distanceBetweenPoints(player.position.x, player.position.y,
                                  vertWallHit.hit.x, vertWallHit.hit.y)
          : SDL_MAX_UINT32;
  if (horzDist < vertDist) {
    ray.distance = horzDist;
    ray.wallHit = horzWallHit.hit;
    ray.wasHitVertical = FALSE;
    ray.wallHitType = horzWallHit.type;
  } else {
    ray.distance = vertDist;
    ray.wallHit = vertWallHit.hit;
    ray.wasHitVertical = TRUE;
    ray.wallHitType = vertWallHit.type;
  }
  rays[columnId] = ray;
}

void castAllRays() {
  double_t initialAngle = player.rotationAngle; // - FOV / 2.f;
  for (int columnId = 0; columnId < NUM_RAYS; columnId++) {
    double_t rayAngle = initialAngle + (double_t)columnId / NUM_RAYS;
    castRay(columnId, rayAngle);
  }
}

void renderRays() {
  for (int columnId = 0; columnId < NUM_RAYS; columnId++) {
    SDL_SetRenderDrawColor(renderer, 0xff, 0x00, 0x00, 255);
    SDL_RenderDrawLine(renderer, player.position.x, player.position.y,
                       rays[columnId].wallHit.x, rays[columnId].wallHit.y);
  }
}

void renderMap() {
  for (int y = 0; y < MAP_NUM_ROWS; y++) {
    for (int x = 0; x < MAP_NUM_COLS; x++) {
      tile_type_t type = map[y][x];
      SDL_Color color;
      switch (type) {
      case WALL:
        SDL_SetRenderDrawColor(renderer, 0xff, 0xff, 0xff, 255);
        break;
      case EMPTY:
        SDL_SetRenderDrawColor(renderer, 0x22, 0x22, 0x22, 255);
        break;
      case CLOSED_DOOR:
        SDL_SetRenderDrawColor(renderer, 0x22, 0x22, 0xff, 255);
        break;
      case OPEN_DOOR:
        SDL_SetRenderDrawColor(renderer, 0x55, 0x55, 0xff, 255);
        break;
      case OUT_OF_BOUNDS:
        break;
      }
      uint32_t tileX = x * TILE_SIZE;
      uint32_t tileY = y * TILE_SIZE;
      SDL_Rect mapTileRect = {
          MINIMAP_SCALE_FACTOR * tileX, MINIMAP_SCALE_FACTOR * tileY,
          MINIMAP_SCALE_FACTOR * TILE_SIZE, MINIMAP_SCALE_FACTOR * TILE_SIZE};
      SDL_RenderFillRect(renderer, &mapTileRect);
    }
  }
}

void processInput() {
  SDL_Event event;
  SDL_PollEvent(&event);
  switch (event.type) {
  case SDL_QUIT:
    isGameRunning = FALSE;
    break;
  case SDL_KEYDOWN:
    if (event.key.keysym.sym == SDLK_ESCAPE) {
      isGameRunning = FALSE;
      break;
    }
    if (event.key.keysym.sym == SDLK_UP) {
      player.walkDirection = 1;
      break;
    }
    if (event.key.keysym.sym == SDLK_DOWN) {
      player.walkDirection = -1;
      break;
    }
    if (event.key.keysym.sym == SDLK_RIGHT) {
      player.turnDirection = 1;
      break;
    }
    if (event.key.keysym.sym == SDLK_LEFT) {
      player.turnDirection = -1;
      break;
    }
  case SDL_KEYUP:
    if (event.key.keysym.sym == SDLK_UP) {
      player.walkDirection = fmax(-1, player.walkDirection - 1);
      break;
    }
    if (event.key.keysym.sym == SDLK_DOWN) {
      player.walkDirection = fmin(1, player.walkDirection + 1);
      break;
    }
    if (event.key.keysym.sym == SDLK_RIGHT) {
      player.turnDirection = fmax(-1, player.turnDirection - 1);
      break;
    }
    if (event.key.keysym.sym == SDLK_LEFT) {
      player.turnDirection = fmin(1, player.turnDirection + 1);
      break;
    }
  }
}

void update() {
  SDL_Delay(FRAME_TIME_LENGTH);

  float deltaTime = (SDL_GetTicks() - ticksLastFrame) / 1000.0f;
  ticksLastFrame = SDL_GetTicks();

  movePlayer(deltaTime);
}

void render() {
  SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
  SDL_RenderClear(renderer);

  renderMap();
  renderRays();
  renderPlayer();
  castAllRays();

  SDL_RenderPresent(renderer);
}

int main(int argc, char *argv[]) {
  isGameRunning = initializeWindow();
  setup();
  while (isGameRunning) {
    processInput();
    update();
    render();
  }
  destroyWindow();
}
