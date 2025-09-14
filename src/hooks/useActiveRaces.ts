import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { ActiveRace } from "../types";

export function useActiveRaces() {
  const [activeRaces, setActiveRaces] = useState<ActiveRace | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const fetchActiveRaces = async () => {
    setLoading(true);
    setError("");
    
    try {
      const result = await invoke<ActiveRace>("get_active_races");
      setActiveRaces(result);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchActiveRaces();
  }, []);

  return {
    activeRaces,
    loading,
    error,
    refetch: fetchActiveRaces,
  };
}