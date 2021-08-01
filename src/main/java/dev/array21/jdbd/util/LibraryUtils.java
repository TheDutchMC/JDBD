package dev.array21.jdbd.util;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Files;
import java.util.regex.Pattern;

import dev.array21.jdbd.exceptions.UnsupportedOperatingSystemException;

public class LibraryUtils {

	public static String getLibraryPath(String name) throws UnsupportedOperatingSystemException {
		String os = System.getProperty("os.name").toLowerCase();
		String libName;
		if(os.contains("linux")) {
			libName = String.format("/x86_64/linux/%s.so", name); 
		} else if(os.contains("windows")) {
			libName = String.format("/x86_64/windows/%s.dll", name);
		} else {
			// TODO support more OSs
			throw new UnsupportedOperatingSystemException("Unsupported OS: " + os);
		}
		
		return libName;	
	}
	
	public static Pair<File, File> saveLibrary(String path) throws IOException {
		String[] nameParts = path.split(Pattern.quote("/"));
		String libName = nameParts[nameParts.length -1];
		
		URL url = LibraryUtils.class.getResource(path);
		File tmpDir = Files.createTempDirectory("jdbd-" + libName).toFile();
		tmpDir.deleteOnExit();
		
		File tmpFile = new File(tmpDir, libName);
		tmpFile.deleteOnExit();
		
		InputStream is = url.openStream();
		Files.copy(is, tmpFile.toPath());
		
		return new Pair<>(tmpDir, tmpFile);
	}
}
