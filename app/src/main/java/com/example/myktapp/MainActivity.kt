package com.example.myktapp

import android.graphics.Bitmap
import android.os.Bundle
import android.widget.ImageView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {

    private external fun renderFractal(bitmap: Bitmap)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        val bitmap = Bitmap.createBitmap(800, 800, Bitmap.Config.ARGB_8888)
        renderFractal(bitmap)
        val imageView: ImageView = findViewById(R.id.imageView)
        imageView.setImageBitmap(bitmap)
    }

    companion object {
        init {
            System.loadLibrary("rust")
        }
    }

}
