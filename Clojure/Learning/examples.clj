(defn fib
  "Naive implementation of a fibonacci function"
  [x]
  (cond
    (= x 0) 0
    (= x 1) 1
    :else (+ (fib (dec x)) (fib (- x 2)))))

(def fast-fib
  "Example that uses memoize to make the recursion faster and avoid repetitive fib calls"
  (memoize
  (fn fast-fib-impl [n]
    (cond
    (= n 0) 0N
    (= n 1) 1N
    :else (+ (fast-fib-impl (- n 1)) (fast-fib-impl (- n 2)))))))

