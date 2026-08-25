package com.simlay.gradle;

import android.app.Activity;
import android.os.Bundle;
import android.widget.LinearLayout;
import android.widget.Button;

public class GradleActivity extends Activity {
    static {
        System.loadLibrary("gradle");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        
        LinearLayout layout = new LinearLayout(this);
        layout.setOrientation(LinearLayout.VERTICAL);
        layout.setLayoutParams(new LinearLayout.LayoutParams(
            LinearLayout.LayoutParams.MATCH_PARENT,
            LinearLayout.LayoutParams.MATCH_PARENT
        ));
        
        Button button = new Button(this);
        button.setText("Show Hello World");
        button.setOnClickListener(v -> {
            showHelloWorld();
        });
        
        layout.addView(button);
        
        setContentView(layout);
    }

    private native void showHelloWorld();
}
