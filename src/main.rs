use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let file: Vec<u64> = fs::read_to_string("input/01")?
        .split('\n')
        .map(|a| a.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let v = (file).iter();
    let v_1 = (file).iter().skip(1);

    let mut count = 0;

    for (i, j) in v.zip(v_1) {
        if j > i {
            count += 1;
        }
    }

    dbg!(count);

    let mut v1 = file.iter().skip(1);
    let mut v2 = file.iter().skip(2);

    let m = file
        .iter()
        .map(|a| a + v1.next().unwrap_or(&0) + v2.next().unwrap_or(&0))
        .collect::<Vec<_>>();

    let t = m.iter();
    let t_1 = m.iter().skip(1);

    let mut count2 = 0;
    for (i, j) in t.zip(t_1) {
        if j > i {
            count2 += 1;
        }
    }
    //part 2
    dbg!(count2);
    Ok(())
}
