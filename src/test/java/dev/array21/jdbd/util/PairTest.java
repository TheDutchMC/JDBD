package dev.array21.jdbd.util;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class PairTest {

	@Test
	public void testPair() {
		Pair<String, String> p = new Pair<>("Foo", "Bar");
		assertEquals(p.getA(), "Foo");
		assertEquals(p.getB(), "Bar");
	}
}
