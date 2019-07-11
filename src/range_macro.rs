#[macro_export]
macro_rules! range {
    () => {{
        Range {
            current: 0,
            target: std::i32::MAX,
            step: 1,
        }
    }};
    (,$end:expr) => {{
        if $end > 0 {
            Range {
                current: 0,
                target: $end,
                step: 1,
            }
        } else {
            Range {
                current: 0,
                target: $end,
                step: -1,
            }
        }
    }};
    (,=$end:expr) => {{
        if $end > 0 {
            Range {
                current: 0,
                target: $end + 1,
                step: 1,
            }
        } else {
            Range {
                current: 0,
                target: $end - 1,
                step: -1,
            }
        }
    }};
    ($start:expr,) => {{
        Range {
            current: $start,
            target: std::i32::MAX,
            step: 1,
        }
    }};
    ($start:expr,$end:expr) => {{
        if $start < $end {
            Range {
                current: $start,
                target: $end,
                step: 1,
            }
        } else {
            Range {
                current: $start,
                target: $end,
                step: -1,
            }
        }
    }};
    ($start:expr,=$end:expr) => {{
        if $start < $end {
            Range {
                current: $start,
                target: $end + 1,
                step: 1,
            }
        } else {
            Range {
                current: $start,
                target: $end - 1,
                step: -1,
            }
        }
    }};
    ($start:expr,$step:expr,$end:expr) => {{
        if ($start < $end && $step < 0) || ($start > $end && $step > 0) {
            panic!("invlid number");
        }
        Range {
            current: $start,
            target: $end,
            step: $step,
        }
    }};
    ($start:expr,$step:expr,=$end:expr) => {{
        if ($start < $end && $step < 0) || ($start > $end && $step > 0) {
            panic!("invlid number");
        }
        let p = if $step > 0 { 1 } else { -1 };
        Range {
            current: $start,
            target: $end + p,
            step: $step,
        }
    }};
    (,$step:expr,$end:expr) => {{
        if ($step > 0 && $end < 0) || ($step < 0 && $end > $step) {
            panic!("invlid number");
        }
        Range {
            current: 0,
            target: $end,
            step: $step,
        }
    }};
    (,$step:expr,=$end:expr) => {{
        if ($step > 0 && $end < 0) || ($step < 0 && $end > $step) {
            panic!("invlid number");
        }
        let p = if $step > 0 { 1 } else { -1 };
        Range {
            current: 0,
            target: $end + p,
            step: $step,
        }
    }};
     ($pattern:expr) => {{
        let mut pattern = $pattern.to_owned();
        if pattern.contains(".."){
            pattern = pattern.replace("..", ",");
        }
        let eq_sign = pattern.contains("=");
        let mut pattern = String::from(pattern);
        if eq_sign {
            pattern.remove(pattern.find("=").unwrap());
        }
        let s: Vec<&str> = pattern
            .split(",")
            .into_iter()
            .filter_map(|x| if x == "" { None } else { Some(x) })
            .collect::<Vec<&str>>();
       if s.len() == 3 {
            let start: i32 = s[0].parse().unwrap();
            let step: i32 = s[1].parse().unwrap();
            let stop: i32 = s[2].parse().unwrap();
            if eq_sign {
                range!(start,step,=stop)
            }else{
                range!(start,step,stop)
            }
        }else if s.len() == 2 {
            if pattern.starts_with(",") {
                let step: i32 = s[0].parse().unwrap();
                let stop: i32 = s[1].parse().unwrap();
                if eq_sign {
                    range!(,step,=stop)
                }else{
                    range!(,step,stop)
                }
            } else {
                let start: i32 = s[0].parse().unwrap();
                let stop: i32 = s[1].parse().unwrap();
                if eq_sign {
                    range!(start,=stop)
                }else{
                    range!(start,stop)
                }
            }
        } else if s.len() == 1 {
            if pattern.starts_with(",") {
                let stop: i32 = s[0].parse().unwrap();
                if eq_sign {
                    range!(,=stop)
                }else{
                    range!(,stop)
                }
            } else {
                let start: i32 = s[0].parse().unwrap();
                range!(start,)
            }
        }else{
            panic!("Invalid Input !");
        }
    }};
}
