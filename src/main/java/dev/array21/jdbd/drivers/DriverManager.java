package dev.array21.jdbd.drivers;

import java.io.File;
import java.io.IOException;

import dev.array21.jdbd.exceptions.UnsupportedOperatingSystemException;
import dev.array21.jdbd.util.LibraryUtils;
import dev.array21.jdbd.util.Pair;

public class DriverManager {
	private static boolean libraryLoaded = false;

	/**
	 * Load the native library
	 * @throws IOException When saving the native library failed
	 * @throws UnsatisfiedLinkError When loading the native library failed
	 * @throws UnsupportedOperatingSystemException When the current operating system is unsupported
	 */
	public static void loadLibrary() throws IOException {
		if(!libraryLoaded) {
			String path = LibraryUtils.getLibraryPath("libjdbd");
			Pair<File, File> filePair = LibraryUtils.saveLibrary(path);
			System.load(filePair.getB().getAbsolutePath());
			libraryLoaded = true;
		}
	}
	
	public static boolean isLoaded() {
		return libraryLoaded;
	}
}
