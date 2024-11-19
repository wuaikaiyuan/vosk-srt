use std::fs::File;
use std::io::Write;
use std::time::Duration;

use vosk::{CompleteResult, CompleteResultMultiple, CompleteResultSingle, Model, Recognizer};
use hound::WavReader;
use anyhow::{Result, Context};

const SAMPLE_RATE: u32 = 16000;

struct SubtitleEntry {
    index: usize,
    start_time: Duration,
    end_time: Duration,
    text: String,
}

impl SubtitleEntry {
    fn to_srt(&self) -> String {
        format!(
            "{}\n{} --> {}\n{}\n\n",
            self.index,
            format_time(self.start_time),
            format_time(self.end_time),
            self.text
        )
    }
}

fn format_time(duration: Duration) -> String {
    let total_millis = duration.as_millis();
    let hours = total_millis / (1000 * 60 * 60);
    let minutes = (total_millis / (1000 * 60)) % 60;
    let seconds = (total_millis / 1000) % 60;
    let millis = total_millis % 1000;
    
    format!("{:02}:{:02}:{:02},{:03}", hours, minutes, seconds, millis)
}

fn process_audio(wav_path: &str, model_path: &str, output_path: &str) -> Result<()> {
    // 加载Vosk模型
    let model = Model::new(model_path).context("Failed to load model")?;
    let mut recognizer = Recognizer::new(&model, SAMPLE_RATE as f32).context("Failed to create recognizer")?;
    
    // 读取WAV文件
    let mut reader = WavReader::open(wav_path).context("Failed to open WAV file")?;
    let samples: Vec<i16> = reader.samples().map(|s| s.unwrap()).collect();
    
    // 设置识别器接收部分结果
    recognizer.set_partial_words(true);
    // recognizer.set_max_alternatives(0);
    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    
    // 进行识别
    let mut subtitles = Vec::new();
    let mut index = 1;
    
    recognizer.accept_waveform(&samples);
    // let result: CompleteResult = recognizer.final_result();
    // let single: CompleteResultSingle<'_> = result.single().unwrap();
    // println!("Full text of the transcript: {:#?}", single.text);
    // single.result.iter().for_each(|word| {
    //     let start = Duration::from_secs_f32(
    //         word.start
    //     );

    //     let end = Duration::from_secs_f32(
    //         word.end
    //     );
    //     let text = word.word.to_string();
        
    //     if !text.is_empty() {
    //         subtitles.push(SubtitleEntry {
    //             index,
    //             start_time: start,
    //             end_time: end,
    //             text,
    //         });
    //         index += 1;
    //     }
    // });

    let result: CompleteResult = recognizer.final_result();
    // let result: CompleteResult = recognizer.result();
    let multiple: CompleteResultMultiple<'_> = result.multiple().unwrap();
    multiple.alternatives.iter().for_each(|alternative| {
        println!("Full transcript text: {:#?}", alternative.text);
        alternative.result.iter().for_each(|word| {
            let start = Duration::from_secs_f32(
                word.start
            );
    
            let end = Duration::from_secs_f32(
                word.end
            );
            let text = word.word.to_string();
            
            if !text.is_empty() {
                subtitles.push(SubtitleEntry {
                    index,
                    start_time: start,
                    end_time: end,
                    text,
                });
                index += 1;
            }
        });
    });    
    
    // 写入SRT文件
    let mut file = File::create(output_path).context("Failed to create output file")?;
    for subtitle in subtitles {
        file.write_all(subtitle.to_srt().as_bytes())?;
    }
    
    Ok(())
}

fn main() -> Result<()> {    
    let output_path = "I:/人生七年/output.srt";
    let wav_path = "I:/人生七年/1.1964.wav";
    let model_path = "I:/.cache/huggingface/hub/vosk-model-en-us-0.22-lgraph";

    process_audio(wav_path, model_path, output_path)?;
    println!("字幕生成完成: {}", output_path);
    
    Ok(())
}