package dev.array21.jdbd.exceptions;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class SqlTypeMismatchExceptionTest {
	@Test
	public void testEmptyConstructor() {
		assertEquals(null, new SqlTypeMismatchException().getMessage());
	}
	
	@Test
	public void testMessageConstructor() {
		assertEquals("foo", new SqlTypeMismatchException("foo").getMessage());
	}
}
