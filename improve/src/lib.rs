use chrono::prelude::*;
use chrono::*;
use failure::Error;

pub fn handle(req : String) -> String {
    let res = handleX(req);
    match res {
        Ok(x) => {
            x
        },
        Err(x) => {
            x.to_string()
        }
    }
}

pub fn handleX(req : String) ->Result<String, Error> {
    let split = req.split(",");
    let vecstr: Vec<&str> = split.collect();
    let dstr1 = vecstr[0];
    let dstr2 = vecstr[1];
    let improve_speed = vecstr[3].parse::<f64>()?;
    let improve_days = vecstr[2].parse::<i64>()?;
    
    let mut d1 = NaiveDate::parse_from_str(dstr1, "%Y-%m-%d")?;
    let mut d2 = NaiveDate::parse_from_str(dstr2, "%Y-%m-%d")?;
    let start = d1.clone();
    
    let mut count = 0;
    let mut improve = 1.0 as f64;
    let dur1 = d2.signed_duration_since(d1);
    let origin_day = dur1.num_days();
    loop {
        let dur1 = d2.signed_duration_since(d1);
        if (dur1.num_seconds() <= 0) {
            break;
        }
        count += 1;
        improve *= improve_speed;
        let m99 = Duration::seconds((dur1.num_seconds() as f64*improve_speed) as i64);
        let d3 = d1.checked_add_signed(m99).unwrap();
        let dur2 = d3.signed_duration_since(d1);
        d1 = d1.checked_add_signed(Duration::days(improve_days)).unwrap();
        d2 = d3;
        //println!("Duration: {:?}, {:?}", dur1, d3);
        //println!("As whole days: {:?}, {:?}", dur1.num_days(), dur2.num_days());
        //println!("Date: {:?}, {:?}", d1, d2);
    }
    let dur2 = d2.signed_duration_since(start);
    let final_day = dur2.num_days();
    let mut res: String = "".to_string();
    res.push_str(&format!("start from {:?}\n", start));
    res.push_str(&format!("end on from {:?}\n", d2));
    res.push_str(&format!("improve count {:?}\n", count));
    res.push_str(&format!("improve from {} days to {} days\n", origin_day, final_day));
    res.push_str(&format!("final performance {:?}\n", improve));
    Ok(res)
}