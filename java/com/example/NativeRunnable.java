package com.example;

public class NativeRunnable implements Runnable {
    private long nativePtr;

    public NativeRunnable(long ptr) {
        this.nativePtr = ptr;
    }

    @Override
    public void run() {
        if (nativePtr != 0) {
            nativeRun(nativePtr);
        }
    }

    // Called when the Runnable is no longer needed
    public void destroy() {
        if (nativePtr != 0) {
            nativeDrop(nativePtr);
            nativePtr = 0;
        }
    }

    private static native void nativeRun(long ptr);
    private static native void nativeDrop(long ptr);
}
