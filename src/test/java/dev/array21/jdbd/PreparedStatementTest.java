package dev.array21.jdbd;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

public class PreparedStatementTest {
	
	@Test
	public void testBindSingleString() {
		PreparedStatement pr = new PreparedStatement("BLABLA ? BLABLA");
		pr.bind(0, "foo");
		assertEquals(pr.getStmt(), "BLABLA foo BLABLA");
	}
	
	@Test
	public void testBindTwoString() {
		PreparedStatement pr = new PreparedStatement("BLABLA ? BLABLA ? BLABLA");
		pr.bind(0, "foo");
		pr.bind(1, "bar");
		
		assertEquals(pr.getStmt(), "BLABLA foo BLABLA bar BLABLA");
	}
	
	@Test
	public void bindInteger() {
		PreparedStatement pr = new PreparedStatement("BLABLA ? BLABLA ? BLABLA");
		pr.bind(0, 1);
		pr.bind(1, 2);
		
		assertEquals(pr.getStmt(), "BLABLA 1 BLABLA 2 BLABLA");
	}
	
	@Test
	public void bindBoolean() {
		PreparedStatement pr = new PreparedStatement("BLABLA ? BLABLA ? BLABLA");
		pr.bind(0, true);
		pr.bind(1, false);
		
		assertEquals(pr.getStmt(), "BLABLA true BLABLA false BLABLA");
	}
	
	@Test
	public void testMixedTypes() {
		PreparedStatement pr = new PreparedStatement("X ? Y ? Z ? T");
		pr.bind(0, true);
		pr.bind(1, 1);
		pr.bind(2, "foo");
		
		assertEquals(pr.getStmt(), "X true Y 1 Z foo T");
	}
	
	@Test
	public void testBeginningOfStatement() {
		PreparedStatement pr = new PreparedStatement("? X");
		pr.bind(0, "foo");
		
		assertEquals(pr.getStmt(), "foo X");
	}
	
	@Test
	public void testEndOfStatement() {
		PreparedStatement pr = new PreparedStatement("X ?");
		pr.bind(0, "foo");
		
		assertEquals(pr.getStmt(), "X foo");
	}
	
	@Test
	public void testAllBound() {
		PreparedStatement pr = new PreparedStatement("??");
		assertFalse(pr.allBound());
		pr.bind(0, "foo");
		assertFalse(pr.allBound());
		pr.bind(1, "bar");
		assertTrue(pr.allBound());
	}
	
	@Test
	public void testNextToOtherBind() {
		PreparedStatement pr = new PreparedStatement("???");
		pr.bind(0, false);
		pr.bind(1, 2);
		pr.bind(2, "foo");
		
		assertEquals(pr.getStmt(), "false2foo");
	}
}
