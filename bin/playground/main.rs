use rand::prelude::Distribution;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::<f32>::new(-2., 2.);

    let rec = rerun::RecordingStreamBuilder::new("ga").spawn()?;

    let mut arr: Vec<(f32, f32, f32)> = vec![(0., 0., 0.); 100_000];

    for i in 0..100_000 {
        arr[i] = (
            dist.sample(&mut rng),
            dist.sample(&mut rng),
            dist.sample(&mut rng),
        );

        let start = std::time::Instant::now();
        // rec.log("dist", &rerun::BarChart::new(arr.as_slice()))?;

        let _ = rec.log("x", &rerun::Points3D::new(arr.to_owned()));


        println!("took = {:?}", start.elapsed());

        std::thread::sleep(std::time::Duration::from_millis(2));
    }

    Ok(())
}