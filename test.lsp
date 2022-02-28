(define func (a b c d)
(begin
	(print a)
	(print b)
	(print c)
	(print d)
	(set a (+ b d))
	(print a)
))

(define func2 (a b)
(begin
	(func a b (+ a 2) (+ b 2))
    (func (* a 2) (* b 2) (* (+ a 2) 2) (* (+ b 2) 2))
))

(func2 5 10)