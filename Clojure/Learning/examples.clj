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

(defn arith-progress [sum x]
  (if (pos? x)
    (recur (+ sum x) (dec x))
    sum))

(defn loop-arith-progress [initial-x]
  (loop [sum 0, x initial-x]
    (if (pos? x)
      (recur (+ sum x) (dec x))
      sum)))

(defn arithmetic-progression
  "Example to illustrate how arity works"
  ([end]
    (/ (*(+ 1 end) end) 2))
  ([start end]
    ( let [num-terms (inc (- end start))]
    (/ (* (+ start end) num-terms) 2)))
  ([start progress num-terms]
    (let [end (+ start (* progress (dec num-terms)))]
    (/ (* (+ start end) num-terms) 2))))

(defn hello
  "Example to illustrate how map destructring works"
  [{:keys [first-name middle-name last-name]
    :or {first-name "sir" middle-name nil last-name "something"}}]
  (let [strings (remove nil? ["Hello," first-name middle-name last-name])]
    (clojure.string/join " " strings)))


