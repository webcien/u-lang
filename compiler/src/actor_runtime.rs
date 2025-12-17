// actor_runtime.rs — U v0.8 Actor Runtime
// MIT License — Copyright (c) 2025 Webcien and U contributors
//
// Micro-runtime for actor-based concurrency with message passing.
// Features:
// - Lightweight mailbox implementation
// - Cooperative scheduling (no OS threads)
// - Message queue management
// - Actor lifecycle management
// - Zero-copy message passing where possible

use std::collections::VecDeque;
use std::fmt;

/// Message type for actor communication
/// In v0.8, messages are serialized as integers and strings
/// In v0.9+, will support generic message types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Integer(i32),
    String(String),
    Call { method: String, args: Vec<Message> },
    Response(Box<Message>),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Integer(n) => write!(f, "{}", n),
            Message::String(s) => write!(f, "\"{}\"", s),
            Message::Call { method, args: _ } => write!(f, "call({})", method),
            Message::Response(msg) => write!(f, "response({})", msg),
        }
    }
}

/// Mailbox for actor message queue
/// Implements FIFO semantics with bounded capacity
#[derive(Debug)]
pub struct Mailbox {
    queue: VecDeque<Message>,
    capacity: usize,
    max_capacity: usize,
}

impl Mailbox {
    /// Create a new mailbox with default capacity (1024 messages)
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            capacity: 0,
            max_capacity: 1024,
        }
    }

    /// Create a mailbox with custom capacity
    pub fn with_capacity(max_capacity: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            capacity: 0,
            max_capacity,
        }
    }

    /// Send a message to the mailbox
    /// Returns Ok(()) if successful, Err if mailbox is full
    pub fn send(&mut self, msg: Message) -> Result<(), String> {
        if self.capacity >= self.max_capacity {
            return Err(format!("Mailbox full: {} messages", self.capacity));
        }
        self.queue.push_back(msg);
        self.capacity += 1;
        Ok(())
    }

    /// Receive the next message from the mailbox
    /// Returns None if mailbox is empty
    pub fn recv(&mut self) -> Option<Message> {
        if let Some(msg) = self.queue.pop_front() {
            self.capacity -= 1;
            Some(msg)
        } else {
            None
        }
    }

    /// Check if mailbox is empty
    pub fn is_empty(&self) -> bool {
        self.capacity == 0
    }

    /// Get current queue size
    pub fn len(&self) -> usize {
        self.capacity
    }

    /// Clear all messages from the mailbox
    pub fn clear(&mut self) {
        self.queue.clear();
        self.capacity = 0;
    }
}

/// Actor state and behavior
/// Represents a single actor instance with its state and mailbox
#[derive(Debug)]
pub struct Actor {
    id: u32,
    name: String,
    mailbox: Mailbox,
    state: ActorState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorState {
    Running,
    Waiting,
    Terminated,
}

impl Actor {
    /// Create a new actor with given name
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            mailbox: Mailbox::new(),
            state: ActorState::Running,
        }
    }

    /// Get actor ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get actor name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get current state
    pub fn state(&self) -> ActorState {
        self.state
    }

    /// Set actor state
    pub fn set_state(&mut self, state: ActorState) {
        self.state = state;
    }

    /// Get mutable reference to mailbox
    pub fn mailbox_mut(&mut self) -> &mut Mailbox {
        &mut self.mailbox
    }

    /// Get immutable reference to mailbox
    pub fn mailbox(&self) -> &Mailbox {
        &self.mailbox
    }

    /// Send a message to this actor
    pub fn send(&mut self, msg: Message) -> Result<(), String> {
        self.mailbox.send(msg)
    }

    /// Receive next message
    pub fn recv(&mut self) -> Option<Message> {
        self.mailbox.recv()
    }

    /// Terminate the actor
    pub fn terminate(&mut self) {
        self.state = ActorState::Terminated;
        self.mailbox.clear();
    }
}

/// Actor runtime scheduler
/// Manages actor lifecycle and cooperative scheduling
pub struct ActorRuntime {
    actors: Vec<Actor>,
    next_id: u32,
    ready_queue: VecDeque<u32>,
    current_actor: Option<u32>,
}

impl ActorRuntime {
    /// Create a new actor runtime
    pub fn new() -> Self {
        Self {
            actors: Vec::new(),
            next_id: 1,
            ready_queue: VecDeque::new(),
            current_actor: None,
        }
    }

    /// Spawn a new actor
    pub fn spawn(&mut self, name: String) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let actor = Actor::new(id, name);
        self.actors.push(actor);
        self.ready_queue.push_back(id);
        id
    }

    /// Send a message to an actor
    pub fn send(&mut self, actor_id: u32, msg: Message) -> Result<(), String> {
        if let Some(actor) = self.actors.iter_mut().find(|a| a.id == actor_id) {
            actor.send(msg)?;
            // Move actor to ready queue if not already there
            if !self.ready_queue.contains(&actor_id) {
                self.ready_queue.push_back(actor_id);
            }
            Ok(())
        } else {
            Err(format!("Actor {} not found", actor_id))
        }
    }

    /// Get next actor to run (cooperative scheduling)
    pub fn next_actor(&mut self) -> Option<u32> {
        while let Some(id) = self.ready_queue.pop_front() {
            if let Some(actor) = self.actors.iter().find(|a| a.id == id) {
                if actor.state == ActorState::Running && !actor.mailbox.is_empty() {
                    self.current_actor = Some(id);
                    return Some(id);
                } else if actor.state == ActorState::Running {
                    // Actor has no messages, put it back in queue
                    self.ready_queue.push_back(id);
                }
            }
        }
        self.current_actor = None;
        None
    }

    /// Process one message for current actor
    pub fn process_message(&mut self) -> Result<Option<Message>, String> {
        if let Some(actor_id) = self.current_actor {
            if let Some(actor) = self.actors.iter_mut().find(|a| a.id == actor_id) {
                if let Some(msg) = actor.recv() {
                    // Put actor back in ready queue if it has more messages
                    if !actor.mailbox.is_empty() {
                        self.ready_queue.push_back(actor_id);
                    }
                    return Ok(Some(msg));
                }
            }
        }
        Ok(None)
    }

    /// Run scheduler until all actors are done
    pub fn run(&mut self) -> Result<(), String> {
        while self.next_actor().is_some() {
            self.process_message()?;
        }
        Ok(())
    }

    /// Get actor by ID
    pub fn get_actor(&self, id: u32) -> Option<&Actor> {
        self.actors.iter().find(|a| a.id == id)
    }

    /// Get mutable actor by ID
    pub fn get_actor_mut(&mut self, id: u32) -> Option<&mut Actor> {
        self.actors.iter_mut().find(|a| a.id == id)
    }

    /// Terminate an actor
    pub fn terminate(&mut self, id: u32) -> Result<(), String> {
        if let Some(actor) = self.get_actor_mut(id) {
            actor.terminate();
            Ok(())
        } else {
            Err(format!("Actor {} not found", id))
        }
    }

    /// Get number of active actors
    pub fn active_actors(&self) -> usize {
        self.actors
            .iter()
            .filter(|a| a.state == ActorState::Running)
            .count()
    }

    /// Get total number of messages in all mailboxes
    pub fn total_messages(&self) -> usize {
        self.actors.iter().map(|a| a.mailbox.len()).sum()
    }

    /// Print runtime statistics
    pub fn print_stats(&self) {
        println!("=== Actor Runtime Statistics ===");
        println!("Total actors: {}", self.actors.len());
        println!("Active actors: {}", self.active_actors());
        println!("Total messages: {}", self.total_messages());
        println!("Ready queue size: {}", self.ready_queue.len());
        for actor in &self.actors {
            println!(
                "  Actor {}: {} (state: {:?}, messages: {})",
                actor.id,
                actor.name,
                actor.state,
                actor.mailbox.len()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mailbox_send_recv() {
        let mut mailbox = Mailbox::new();
        let msg = Message::Integer(42);
        assert!(mailbox.send(msg.clone()).is_ok());
        assert_eq!(mailbox.len(), 1);
        assert_eq!(mailbox.recv(), Some(msg));
        assert_eq!(mailbox.len(), 0);
    }

    #[test]
    fn test_mailbox_capacity() {
        let mut mailbox = Mailbox::with_capacity(2);
        assert!(mailbox.send(Message::Integer(1)).is_ok());
        assert!(mailbox.send(Message::Integer(2)).is_ok());
        assert!(mailbox.send(Message::Integer(3)).is_err());
    }

    #[test]
    fn test_actor_creation() {
        let actor = Actor::new(1, "TestActor".to_string());
        assert_eq!(actor.id(), 1);
        assert_eq!(actor.name(), "TestActor");
        assert_eq!(actor.state(), ActorState::Running);
    }

    #[test]
    fn test_runtime_spawn() {
        let mut runtime = ActorRuntime::new();
        let id1 = runtime.spawn("Actor1".to_string());
        let id2 = runtime.spawn("Actor2".to_string());
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(runtime.active_actors(), 2);
    }

    #[test]
    fn test_runtime_send_recv() {
        let mut runtime = ActorRuntime::new();
        let actor_id = runtime.spawn("TestActor".to_string());
        let msg = Message::Integer(42);
        assert!(runtime.send(actor_id, msg.clone()).is_ok());
        assert_eq!(runtime.get_actor(actor_id).unwrap().mailbox.len(), 1);
    }
}
