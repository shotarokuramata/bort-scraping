interface PerformanceItemProps {
  label: string;
  value: number | undefined;
  format?: 'percentage' | 'decimal';
}

export function PerformanceItem({ label, value, format = 'percentage' }: PerformanceItemProps) {
  if (value === undefined) return null;

  const formattedValue = format === 'percentage' 
    ? `${(value * 100).toFixed(1)}%` 
    : value.toFixed(2);

  return (
    <div className="perf-item">
      <span>{label}:</span> {formattedValue}
    </div>
  );
}