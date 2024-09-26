use anyhow::Result;
use plotly::{common::Mode, Layout, Plot, Scatter};
use core::f64;
use std::{
    io::{stdin, Write},
};
use tokio::time::{self, Duration};
use timegrapher::audio::io as audioio;
use timegrapher::signal::fft::lowpass_filter;


#[tokio::main]
async fn main() -> Result<()> {
    let cons = audioio::get_connectors()?;
    // println!("Connectors are {:#?}", cons);

    let mut audiostream =
        audioio::AudioStreamBuilder::new(&cons[0], &"MacBook Pro Microphone".to_string())?
            .build()?;


    let track  = audiostream.get_track_by_duration(Duration::from_secs(4)).await;

    //version with independant process \
    // create file and the writer stream to file
    // let auidiofile = File::create("test.txt").expect("Unable to create file");
    // let mut writer = std::io::BufWriter::new(auidiofile);
    // tokio::spawn(async move {
    // let mut counter: u32 = 0;
    // while let Some((time, value)) = audiostream.next().await {
    //    writeln!(writer, "{}, {}", time, value).expect("Error writing data");
    // }
    // writer.flush().expect("Unable to flush");
    // drop(audiostream);
    // });
    // // Simulate running for 5 seconds before exiting
    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    // println!("Audio track saved to test.txt");

    // let x: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
    // let y: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));

    // let x_c = Arc::clone(&x);
    // let y_c = Arc::clone(&y);

    // let handle = tokio::spawn(async move {
    //     let result = time::timeout(Duration::from_secs(5), async {
    //         let mut x = x_c.lock().await;
    //         let mut y = y_c.lock().await;

    //         // for _ in 0..100 {
    //         //     match audiostream.next().await{
    //         //         Some((time, value)) => {
    //         //             x.push(time);
    //         //             y.push(value);
    //         //         },
    //         //         None => break
    //         //     }
    //         // }
    //         while let Some((time, value)) = audiostream.next().await {
    //             x.push(time);
    //             y.push(value);
    //             // if counter % 5000 == 0 {
    //             //     println!("on {:}", counter)
    //             // }
    //             // counter += 1;
    //         }

    //         drop(audiostream);
    //     }).await;

    //     match result {
    //         Ok(_) => println!("Process completed within the timeout"),
    //         Err(_) => println!("Process was interrupted after 5 seconds"),
    //     }
    // });

    // // Wait for the spawned task to complete
    // handle.await.unwrap();

    // handling data
    // let x = x.lock().await.to_vec();
    // let y = y.lock().await.to_vec();

    let br = track.samplerate.clone();
    let x = track.get_time();
    let y = lowpass_filter(track, 100.into());


    // let min = y.iter().cloned().fold(f64::INFINITY, f64::min);
    // let max = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // // normalising
    // let mut yn: Vec<f64> = Vec::new();
    // y.iter().for_each(|&yy| {
    //     yn.push((yy - min) / (max - min));
    //  });

    let trace = Scatter::new(x, y).mode(Mode::Lines);
    let layout = Layout::new()
        .title("Dynamic Plot")
        .x_axis(plotly::layout::Axis::new().title("Time (s)"))
        .y_axis(plotly::layout::Axis::new().title("Amplitude"));

    // Create a Plotly plot
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);


    plot.show();
                   
    Ok(())
}
