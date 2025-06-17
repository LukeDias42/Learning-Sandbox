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

(defn game-loop [initial-board rows cols]
  (clear-screen)
    (reset-cursor)
    (print-board board rows cols)

(defn game-of-life [rows cols initial-alive-percentage]
  (print "\u001b[?25l")
  (let [board (make-randomized-board rows cols initial-alive-percentage)]
    (game-loop board rows cols)))

(game-of-life 38 170 10)

