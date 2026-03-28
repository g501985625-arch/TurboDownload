use super::DownloadEvent;

/// Event emitter for download events
pub struct EventEmitter {
    task_id: String,
}

impl EventEmitter {
    /// Create a new event emitter for a task
    pub fn new(task_id: String) -> Self {
        Self { task_id }
    }
    
    /// Emit download started event
    pub fn started(&self, total_size: u64) -> DownloadEvent {
        DownloadEvent::Started {
            task_id: self.task_id.clone(),
            total_size,
        }
    }
    
    /// Emit progress event
    pub fn progress(
        &self,
        downloaded: u64,
        speed: u64,
        percent: f64,
        eta: Option<u64>,
    ) -> DownloadEvent {
        DownloadEvent::Progress {
            task_id: self.task_id.clone(),
            downloaded,
            speed,
            percent,
            eta,
        }
    }
    
    /// Emit chunk completed event
    pub fn chunk_completed(&self, chunk_id: u32) -> DownloadEvent {
        DownloadEvent::ChunkCompleted {
            task_id: self.task_id.clone(),
            chunk_id,
        }
    }
    
    /// Emit download completed event
    pub fn completed(&self, file_path: String) -> DownloadEvent {
        DownloadEvent::Completed {
            task_id: self.task_id.clone(),
            file_path,
        }
    }
    
    /// Emit download failed event
    pub fn failed(&self, error: String) -> DownloadEvent {
        DownloadEvent::Failed {
            task_id: self.task_id.clone(),
            error,
        }
    }
    
    /// Emit download paused event
    pub fn paused(&self) -> DownloadEvent {
        DownloadEvent::Paused {
            task_id: self.task_id.clone(),
        }
    }
    
    /// Emit download resumed event
    pub fn resumed(&self) -> DownloadEvent {
        DownloadEvent::Resumed {
            task_id: self.task_id.clone(),
        }
    }
    
    /// Emit download cancelled event
    pub fn cancelled(&self) -> DownloadEvent {
        DownloadEvent::Cancelled {
            task_id: self.task_id.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_emitter() {
        let emitter = EventEmitter::new("test-task".to_string());
        
        let event = emitter.started(1000);
        assert_eq!(event.task_id(), "test-task");
        
        let event = emitter.progress(500, 100, 50.0, Some(5));
        assert_eq!(event.task_id(), "test-task");
        
        let event = emitter.completed("/tmp/test.txt".to_string());
        assert_eq!(event.task_id(), "test-task");
    }
}
