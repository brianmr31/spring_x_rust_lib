package corp.valhalla.web.lib;

import java.util.List;

public class HelloWorld {
    
	public native String check(String input);
	public native void loadExecl(String file, String sheet, String sperator) throws Exception;
	public native void close();
	public native long getRowTotal();
	public native String getTypeHeaders();
	public native String getNameHeaders();
	public native long getHeaderTotal();
	public native String getRow(int i);
	
	public native int[] getTestArray( List<String> tests );
	
	static {
		// System.setProperty("java.library.path", "/home/brian/Downloads/web/rust/readExcel/target/debug");
        System.load("/home/brian/eclipse-workspace/web/rust/readExcel/target/release/libreadExcel.so");
	}
}
