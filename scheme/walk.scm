;; walk.scm

(define w (make-walk))
(define omega (make-rng 10031))

(for-each (lambda (_) (step! w (random-step omega))) (range 0 10))

(walk-last w)
(walk-length w)
(walk-path w)

;; end
