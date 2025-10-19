import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AllVenuesResponse } from "../types";

export function useActiveRaces() {
  const [allVenues, setAllVenues] = useState<AllVenuesResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const fetchAllVenues = async () => {
    setLoading(true);
    setError("");
    
    try {
      const result = await invoke<AllVenuesResponse>("get_all_venues_with_status");
      setAllVenues(result);
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchAllVenues();
  }, []);

  return {
    allVenues,
    loading,
    error,
    refetch: fetchAllVenues,
  };
}