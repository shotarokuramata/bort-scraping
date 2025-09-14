interface StatItemProps {
  label: string;
  value: string | number;
  className?: string;
}

export function StatItem({ label, value, className = "stat-item" }: StatItemProps) {
  return (
    <div className={className}>
      <span className="stat-label">{label}:</span>
      <span className="stat-value">{value}</span>
    </div>
  );
}