(defn fib
  "Naive implementation of a fibonacci function"
  [x]
  (cond
    (= x 0) 0
    (= x 1) 1
    :else (+ (fib (dec x)) (fib (- x 2)))))

