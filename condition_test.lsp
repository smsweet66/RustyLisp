(print (cons 1 ()))
(set x (cons (cons (cons 1 ()) 2) 3))
(set y x)
(while (list? y)
(begin
	(print y)
	(set y (car y))
))

(print y)
(print (null? ()))
(set y x)
(while (= (null? y) ())
(begin
	(print y)
	(set y (cdr y))
))

(print y)