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

(defn tail-call-fib
  "Example that uses tail call optimization for calculate the nth fibonacci value."
  [initial-x]
    (loop [a 0N b 1N x initial-x]
      (if (pos? x)
          (recur b (+ a b) (dec x))
      a)))

