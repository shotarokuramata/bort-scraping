interface HelloMessageProps {
  message: string;
}

export function HelloMessage({ message }: HelloMessageProps) {
  if (!message) return null;

  return (
    <div className="hello-message">
      <h2>Backend Message: {message}</h2>
    </div>
  );
}