use std::marker::PhantomData;
/// used to designat an Object stored in the RessourcesHolder
pub struct Handle<T>
{
    id: u64,
    index: usize,
    phantom: PhantomData<T> // for genericity and security
}
use std::fmt;
impl<T> fmt::Debug for Handle<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
	write!(f, "Handle{{id: {}, index: {}}}", self.id, self.index)
    }
}

impl<T> PartialEq for Handle<T>
{
    fn eq(&self, other: &Self) -> bool
    {
	self.index == other.index
    }
}

impl<T> Eq for Handle<T>{}

use std::hash::{Hash, Hasher};
impl<T> Hash for Handle<T>
{
    fn hash<H: Hasher>(&self, hasher: &mut H)
    {
	self.index.hash(hasher);
    }
}

impl<T> Clone for Handle<T>
{
    fn clone(&self) -> Self
    {
	Self
	{
	    id: self.id,
	    index: self.index,
	    phantom: PhantomData
	}
    }
}
impl<T> Copy for Handle<T>{}

// we have to precise it is safe to share between threads (because of the PhantomData)
unsafe impl<T> Sync for Handle<T>{}
unsafe impl<T> Send for Handle<T>{}

#[derive(Debug)]
pub struct Register<T>
{
    generation: u64,
    storage: Vec<T>,
    alive: Vec<bool>,
    free: Vec<usize>
}

impl<T> Register<T>
{
    pub fn new() -> Self
    {
	Self
	{
	    generation: 0,
	    storage: Vec::new(),
	    alive: Vec::new(),
	    free: Vec::new()
	}
    }

    pub fn add(&mut self, thing: T) -> Handle<T>
    {
	let handle = match self.free.pop()
	{
	    None =>
	    {
		self.storage.push(thing);
		self.alive.push(true);
		Handle
		{
		    id: self.generation,
		    index: self.storage.len()-1,
		    phantom: PhantomData
		}
	    },
	    Some(index) =>
	    {
		self.storage[index] = thing;
		self.alive[index] = true;
		Handle
		{
		    id: self.generation,
		    index: index,
		    phantom: PhantomData
		}
	    }
	};
	self.generation+=1;
	handle
    }

    pub fn remove(&mut self, handle: Handle<T>)
    {
	let index = handle.index;
	if self.alive[index]
	{
	    self.alive[index] = false;
	    self.free.push(index)
	}
    }

    pub fn get(&self, handle: Handle<T>) -> &T
    {
	let index = handle.index;
	unsafe{self.storage.get_unchecked(index)}
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> &mut T
    {
	let index = handle.index;
	unsafe{self.storage.get_unchecked_mut(index)}
    }

    
}

