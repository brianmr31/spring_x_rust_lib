package corp.valhalla.web.lib;

public class PosgressRust {

	public native void addList(String data);
	public native void runBatch();
	public native void close();
	
	static {
        System.load("/home/brian/eclipse-workspace/web/rust/posgres_rust/target/release/libposgres_rust.so");
	}
}
