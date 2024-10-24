#define TRUE 1
#define FALSE 0

#define PI 3.14159265
#define TWO_PI 6.28318530
#define to_radian(a) (a * PI / 180)
#define to_angle(a) (a * 180 / PI)

#define TILE_SIZE 64
#define MAP_NUM_ROWS 13
#define MAP_NUM_COLS 20
#define MINIMAP_SCALE_FACTOR 1.0

#define WINDOW_WIDTH (MAP_NUM_COLS * TILE_SIZE)
#define WINDOW_HEIGHT (MAP_NUM_ROWS * TILE_SIZE)

#define FOV (to_radian(60))
#define NUM_RAYS WINDOW_WIDTH

#define FPS 60
#define FRAME_TIME_LENGTH (1000 / FPS)
