package dev.array21.jdbd.exceptions;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class DriverUnloadedExceptionTest {

	@Test
	public void testEmptyConstructor() {
		assertEquals(null, new DriverUnloadedException().getMessage());
	}
	
	@Test
	public void testMessageConstructor() {
		assertEquals("foo", new DriverUnloadedException("foo").getMessage());
	}
}
