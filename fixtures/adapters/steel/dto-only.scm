;; DTO-only Steel probe fixture. It is source-envelope evidence for the bounded
;; adapter profile; default validation never executes this source.
(define (probe input)
  (kamacite.add-one input))
(provide probe)
