import "package:flutter/material.dart";

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    const theme = AppBarTheme(
      backgroundColor: Colors.white,
      foregroundColor: Colors.black,
    );

    return MaterialApp(
        title: "phisyRC",
        theme: ThemeData(appBarTheme: theme),
        home: const MyHomePage());
  }
}

class MyHomePage extends StatelessWidget {
  const MyHomePage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("phisyRC"),
      ),
      body: const Center(
        child: Text("Hello, world!"),
      ),
    );
  }
}
