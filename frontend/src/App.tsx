import { BeakerIcon } from '@heroicons/react/24/solid'

import './App.css';

function App() {
  return (
    <div className="min-h-screen grid grid-cols-1 place-items-center justify-items-center mx-auto py-8">
      <div className="text-2xl font-bold font-mono">
        <h1 className="content-center">Vite + React + TS + Tailwind</h1>
      </div>
      <div className="w-fit max-w-md">
        <button className="btn">
          <BeakerIcon className="size-8" />
          Button
        </button>
        <button className="btn btn-neutral">Neutral</button>
        <button className="btn btn-primary">Primary</button>
        <button className="btn btn-secondary">Secondary</button>
        <button className="btn btn-accent">Accent</button>
        <button className="btn btn-ghost">Ghost</button>
        <button className="btn btn-link">Link</button>
      </div>
    </div>
  )
}

export default App
