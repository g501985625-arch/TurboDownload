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
    fn test_event_emitter_creation() {
        let emitter = EventEmitter::new("test-task".to_string());
        let event = emitter.started(1000);
        
        assert_eq!(event.task_id(), "test-task");
        assert!(matches!(event, DownloadEvent::Started { total_size: 1000, .. }));
    }

    #[test]
    fn test_started_event() {
        let emitter = EventEmitter::new("task-123".to_string());
        let event = emitter.started(1024 * 1024);
        
        assert!(matches!(event, DownloadEvent::Started { 
            task_id, 
            total_size: 1048576 
        } if task_id == "task-123"));
    }

    #[test]
    fn test_progress_event() {
        let emitter = EventEmitter::new("task-456".to_string());
        let event = emitter.progress(500, 100, 50.0, Some(5));
        
        assert!(matches!(event, DownloadEvent::Progress { 
            task_id,
            downloaded: 500,
            speed: 100,
            percent: 50.0,
            eta: Some(5)
        } if task_id == "task-456"));
    }

    #[test]
    fn test_chunk_completed_event() {
        let emitter = EventEmitter::new("task-789".to_string());
        let event = emitter.chunk_completed(42);
        
        assert!(matches!(event, DownloadEvent::ChunkCompleted { 
            task_id,
            chunk_id: 42 
        } if task_id == "task-789"));
    }

    #[test]
    fn test_completed_event() {
        let emitter = EventEmitter::new("task-abc".to_string());
        let event = emitter.completed("/downloads/file.zip".to_string());
        
        assert!(matches!(event, DownloadEvent::Completed { 
            task_id,
            file_path 
        } if task_id == "task-abc" && file_path == "/downloads/file.zip"));
    }

    #[test]
    fn test_failed_event() {
        let emitter = EventEmitter::new("task-def".to_string());
        let event = emitter.failed("Network timeout".to_string());
        
        assert!(matches!(event, DownloadEvent::Failed { 
            task_id,
            error 
        } if task_id == "task-def" && error == "Network timeout"));
    }

    #[test]
    fn test_paused_event() {
        let emitter = EventEmitter::new("task-pause".to_string());
        let event = emitter.paused();
        
        assert!(matches!(event, DownloadEvent::Paused { task_id } if task_id == "task-pause"));
    }

    #[test]
    fn test_resumed_event() {
        let emitter = EventEmitter::new("task-resume".to_string());
        let event = emitter.resumed();
        
        assert!(matches!(event, DownloadEvent::Resumed { task_id } if task_id == "task-resume"));
    }

    #[test]
    fn test_cancelled_event() {
        let emitter = EventEmitter::new("task-cancel".to_string());
        let event = emitter.cancelled();
        
        assert!(matches!(event, DownloadEvent::Cancelled { task_id } if task_id == "task-cancel"));
    }

    #[test]
    fn test_event_serialization() {
        use serde_json;
        
        let emitter = EventEmitter::new("serialize-test".to_string());
        let event = emitter.progress(1024, 512, 25.5, Some(10));
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"progress\""));
        assert!(json.contains("\"task_id\":\"serialize-test\""));
        assert!(json.contains("\"downloaded\":1024"));
        
        // Deserialize
        let deserialized: DownloadEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.task_id(), "serialize-test");
    }
}
