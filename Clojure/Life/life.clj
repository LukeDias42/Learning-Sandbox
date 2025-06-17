(ns life)

(defn print-board [board rows cols]
    (loop [i 0]
      (if (< i (* rows cols))
        (do
        (if (and (not (= i 0)) (= (mod i cols) 0))
          (println))
        (print (or (and (aget board i) "█") " "))
        (recur (inc i)))))
    (println)
    (flush))

(defn make-randomized-board [rows cols percent]
    (let [board (make-array Boolean/TYPE (* rows cols))]
    (loop [i 0]
      (if (< i (* rows cols))
        (do
          (if (< (rand 100.0) percent)
            (aset board i true))
        (recur (inc i)))))
    board))

(defn clear-screen []
  (print "\u001b[2J\u001b[H")
  (flush))

(defn reset-cursor []
  (print "\u001b[H")
  (flush))

(defn wait-ms [ms]
  (Thread/sleep ms))

(defn count-neighbors [board rows cols pos]
  (let [x (mod pos cols) y (quot pos cols)]
    (count (filter true? [
            (and
                (and (> x 0) (> y 0))
                (aget board (dec (- pos cols))))
            (and
                  (> y 0)
                  (aget board (- pos cols)))
            (and
                  (and (< x (dec cols)) (> y 0))
                  (aget board (inc (- pos cols))))
            (and
                  (> x 0)
                  (aget board (dec pos)))
            (and
                  (< x (dec cols))
                  (aget board (inc pos)))
            (and
                  (and (> x 0) (< y (dec rows)))
                  (aget board (dec (+ pos cols))))
            (and
                  (< y (dec rows))
                  (aget board (+ pos cols)))
            (and
                  (and (< x (dec cols)) (< y (dec rows)))
                  (aget board (inc (+ pos cols))))])))) ; down-right

; alive with < 2 neighbors - dies
; alive with 2 or 3 neighbors - lives
; alive with > 3 neighbors - dies
; dead with = 3 neighbors - becomes alive
(defn update-board [board rows cols]
  (let [new-board (make-array Boolean/TYPE (* rows cols))]
    (loop [i 0]
      (if (< i (* rows cols))
        (let [neighbors (count-neighbors board rows cols i) alive (aget board i)]
          ; (println "pos: " i " neighbors: " neighbors " is alive: " alive)
          (aset new-board i (or
                (and alive (and (> neighbors 1) (< neighbors 4)))
                (and (not alive) (= neighbors 3))))
        (recur (inc i)))))
    new-board))

(defn game-loop [initial-board rows cols]
  (clear-screen)
  (loop [board initial-board]
    (reset-cursor)
    (print-board board rows cols)
    (recur (update-board board rows cols))))

(defn game-of-life [rows cols initial-alive-percentage]
  (print "\u001b[?25l")
  (let [board (make-randomized-board rows cols initial-alive-percentage)]
    (game-loop board rows cols)))

(game-of-life 38 170 10)

