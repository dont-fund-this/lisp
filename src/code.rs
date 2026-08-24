pub const CODE: &str = r#"(define thingy
  (list (hash "sid" 1 "tag" "A" "code" 95000)
        (hash "sid" 2 "tag" "B" "code" 75000)
        (hash "sid" 3 "tag" "A" "code" 60000)))

(filter (fn (x) (equal? (hash-ref x "tag") "A")) thingy)
"#;
