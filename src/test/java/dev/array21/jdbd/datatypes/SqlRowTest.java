package dev.array21.jdbd.datatypes;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNull;

import org.junit.Before;
import org.junit.Test;

import dev.array21.jdbd.exceptions.SqlTypeMismatchException;

public class SqlRowTest {

	private SqlRow row;
	
	@Before
	public void setup() {
		this.row = new SqlRow(
				new String[] { "a", "b", "c", "d", "e" },
				new Object[] { "baz", 1, 2.5d, false, new Byte[] { 0x0 }},
				new Class<?>[] { String.class, Integer.class, Double.class, Boolean.class, Byte[].class });
	}
	
	@Test(expected = IllegalArgumentException.class)
	public void testInvalidLengthNameObjs() {
		new SqlRow(new String[1], new Object[0], new Class<?>[1]);
	}
	
	@Test(expected = IllegalArgumentException.class)
	public void testInvalidLengthObjsClasses() {
		new SqlRow(new String[1], new Object[1], new Class<?>[2]);
	}
	
	@Test
	public void testGetString() {
		assertEquals("baz", this.row.getString("a"));
	}
	
	@Test
	public void testGetInt() {
		assertEquals(Integer.valueOf(1), this.row.getInt("b"));
	}
	
	@Test
	public void testGetDouble() {
		assertEquals(Double.valueOf(2.5d), this.row.getDouble("c"));
	}
	
	@Test
	public void testGetBoolean() {
		assertEquals(Boolean.valueOf(false), this.row.getBoolean("d"));
	}
	
	@Test
	public void testGetByteArray() {
		assertArrayEquals(new Byte[] { 0x0 }, this.row.getBytes("e"));
	}
	
	@Test
	public void testGetNonexistentColumn() {
		assertNull(this.row.getBoolean("z"));
	}
	
	@Test(expected = SqlTypeMismatchException.class)
	public void testGetWrongType() {
		this.row.getString("e");
	}
}
