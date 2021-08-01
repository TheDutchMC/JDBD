package dev.array21.jdbd.util;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import java.io.File;
import java.io.IOException;

import org.junit.Test;

import dev.array21.jdbd.exceptions.UnsupportedOperatingSystemException;

public class LibraryUtilsTest {

	@Test
	public void testCreation() {
		assertEquals(new LibraryUtils().getClass(), LibraryUtils.class);
	}
	
	@Test
	public void testGetLibraryPath() {
		System.setProperty("os.name", "linux");
		assertEquals("/x86_64/linux/libfoo.so", LibraryUtils.getLibraryPath("libfoo"));
		
		System.setProperty("os.name", "windows");
		assertEquals("/x86_64/windows/libfoo.dll", LibraryUtils.getLibraryPath("libfoo"));
	}
	
	@Test(expected = UnsupportedOperatingSystemException.class)
	public void testGetLibraryPathUnsupportedOs() {
		System.setProperty("os.name", "foo");
		LibraryUtils.getLibraryPath("libfoo");
	}
	
	@Test
	public void testSaveLibrary() throws IOException {
		Pair<File, File> filePair =  LibraryUtils.saveLibrary("/META-INF/MANIFEST.MF");
		assertTrue(filePair.getA().isDirectory());
		assertTrue(filePair.getB().isFile());
		assertTrue(filePair.getB().getAbsolutePath().contains("MANIFEST.MF"));
		
		filePair.getA().delete();
		filePair.getB().delete();
	}
}
