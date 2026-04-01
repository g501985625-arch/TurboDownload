use std::fs;
use turbo_crawler::{ResourceExtractor, ResourceType};
use turbo_crawler::extractor::{StreamFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取测试HTML文件
    let html_content = fs::read_to_string("/Users/macipad/Desktop/TurboDownload/code/TurboDownload/test_radars.html")?;
    
    // 创建资源提取器
    let extractor = ResourceExtractor::new("https://example.com");
    
    // 提取所有资源
    let resources = extractor.extract(&html_content)?;
    
    println!("=== 雷达功能测试结果 ===");
    println!("总共找到 {} 个资源\n", resources.len());
    
    // 统计不同类型的资源
    let mut image_count = 0;
    let mut video_count = 0;
    let mut audio_count = 0;
    let mut streaming_count = 0;
    let mut blob_count = 0;
    let mut lazy_image_count = 0;
    let mut bg_image_count = 0;
    let mut iframe_count = 0;
    
    for resource in &resources {
        match &resource.resource_type {
            ResourceType::Image => {
                image_count += 1;
                println!("🖼️ 图片: {}", resource.url);
            },
            ResourceType::Video => {
                video_count += 1;
                println!("🎬 视频: {}", resource.url);
            },
            ResourceType::Audio => {
                audio_count += 1;
                println!("🎵 音频: {}", resource.url);
            },
            ResourceType::Streaming => {
                streaming_count += 1;
                if let Some(format) = &resource.stream_format {
                    println!("📡 流媒体({}): {}", 
                             match format {
                                 StreamFormat::HLS => "HLS",
                                 StreamFormat::DASH => "DASH", 
                                 StreamFormat::SmoothStreaming => "SmoothStreaming",
                                 StreamFormat::Unknown => "Unknown"
                             }, 
                             resource.url);
                } else {
                    println!("📡 流媒体: {}", resource.url);
                }
            },
            ResourceType::Other(url) if url.starts_with("blob:") => {
                blob_count += 1;
                println!("🔗 Blob URL: {}", url);
            },
            _ => {
                if resource.url.contains("youtube.com") || resource.url.contains("bilibili.com") {
                    iframe_count += 1;
                    println!("📺 视频嵌入: {}", resource.url);
                } else if resource.url.contains("/images/") && 
                         (resource.url.contains("lazy") || resource.url.contains("original")) {
                    lazy_image_count += 1;
                    println!("😴 懒加载图片: {}", resource.url);
                } else if resource.url.contains("background") {
                    bg_image_count += 1;
                    println!("🎨 背景图片: {}", resource.url);
                } else {
                    println!("📄 其他: {:?} - {}", resource.resource_type, resource.url);
                }
            }
        }
    }
    
    println!("\n=== 统计结果 ===");
    println!("图片数量: {}", image_count);
    println!("视频数量: {}", video_count);
    println!("音频数量: {}", audio_count);
    println!("流媒体数量: {}", streaming_count);
    println!("Blob URL数量: {}", blob_count);
    println!("懒加载图片数量: {}", lazy_image_count);
    println!("背景图片数量: {}", bg_image_count);
    println!("视频嵌入数量: {}", iframe_count);
    
    println!("\n=== 资源类型分布 ===");
    let total_resources = resources.len();
    println!("图片: {:.1}%", (image_count as f64 / total_resources as f64) * 100.0);
    println!("视频: {:.1}%", (video_count as f64 / total_resources as f64) * 100.0);
    println!("音频: {:.1}%", (audio_count as f64 / total_resources as f64) * 100.0);
    println!("流媒体: {:.1}%", (streaming_count as f64 / total_resources as f64) * 100.0);
    
    // 检查是否至少有5种资源类型被识别
    let has_images = image_count > 0;
    let has_videos = video_count > 0;
    let has_audios = audio_count > 0;
    let has_streaming = streaming_count > 0;
    let has_lazy_images = lazy_image_count > 0;
    let has_bg_images = bg_image_count > 0;
    let has_blobs = blob_count > 0;
    let has_iframes = iframe_count > 0;
    
    let detected_types = [
        has_images, has_videos, has_audios, has_streaming, 
        has_lazy_images, has_bg_images, has_blobs, has_iframes
    ].iter().filter(|&&x| x).count();
    
    println!("\n=== 验证结果 ===");
    println!("检测到的资源类型数量: {}/8", detected_types);
    if detected_types >= 5 {
        println!("✅ 验证通过: 检测到至少5种资源类型 ({})", detected_types);
    } else {
        println!("❌ 验证未通过: 检测到少于5种资源类型 ({})", detected_types);
    }
    
    Ok(())
}