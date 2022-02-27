(print (null? x))
(set x (cons 1.5 (cons 3 ())))
(print x)
(print (null? x))
(print (list? x))
(print (car x))
(print (cdr x))
(print (symbol? x))
(print (symbol? (cons 1.5 (cons 3 4))))

(define f1 (a b c)
(print "fuck off"))

(f1 (+ 1 9) 5 "no")