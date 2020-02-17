package io.waweb.cartoonifyit;

import android.os.Bundle;
import android.widget.TextView;

import androidx.appcompat.app.AppCompatActivity;


/**
 * @author Aaron R Miller<aaron.miller@waweb.io>
 */
public class MainActivity extends AppCompatActivity {

    private static final String TAG = "MainActivity";

    // Used to load the 'rust' library on application startup.
    static {
        System.loadLibrary("native_app");
    }

    TextView textView;

    private long _appPtr;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        textView = findViewById(R.id.sample_text);

        // Example of a call to a native method
        _appPtr = createNativeApp("World");
        appendToTextView(sayHello());
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        destroyNativeApp(_appPtr);
    }

    private static native String greeting(long appPtr);

    private static native long createNativeApp(final String to);

    private static native String destroyNativeApp(long appPtr);

    public String sayHello() {
        return greeting(_appPtr);
    }

    public void appendToTextView(String string) {
        textView.append(string + "\n");
    }
}
