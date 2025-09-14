import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export function useHelloWorld() {
  const [helloMessage, setHelloMessage] = useState("");

  useEffect(() => {
    async function fetchHelloWorld() {
      try {
        const result = await invoke<string>("hello_world");
        setHelloMessage(result);
      } catch (err) {
        console.error("Hello World fetch error:", err);
      }
    }
    
    fetchHelloWorld();
  }, []);

  return helloMessage;
}