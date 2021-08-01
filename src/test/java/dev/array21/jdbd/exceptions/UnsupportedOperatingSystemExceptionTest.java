package dev.array21.jdbd.exceptions;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class UnsupportedOperatingSystemExceptionTest {
	@Test
	public void testEmptyConstructor() {
		assertEquals(null, new UnsupportedOperatingSystemException().getMessage());
	}
	
	@Test
	public void testMessageConstructor() {
		assertEquals("foo", new UnsupportedOperatingSystemException("foo").getMessage());
	}
}
