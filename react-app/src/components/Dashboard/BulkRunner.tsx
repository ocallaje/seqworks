import { useState, useEffect } from 'react';
import Card from './Card';
import { initPipe, getProjectList } from '../../backend';
import { useContext } from 'react';
import { StatusContext } from '../../store/StatusContext';

export default function BulkRunner() {
  const [form, setForm] = useState({
    customRunName: '',
    project: '',
  });

  const [projects, setProjects] = useState<string[]>([]);
  const { setRunStatus } = useContext(StatusContext);

  useEffect(() => {
    async function fetch() {
      const list = await getProjectList('bulk');
      setProjects(list);
    }
    fetch();
  }, []);

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>
  ) => {
    const { name, value } = e.target;
    setForm(prev => ({ ...prev, [name]: value }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setRunStatus('running');
    await initPipe({ wrapper: { params: { AppParams: form } } });
  };

  return (
    <Card>
      <h2 className="text-lg font-medium mb-4 text-primary">
        Submit Bulk RNA‑seq Run
      </h2>
      <form onSubmit={handleSubmit} className="grid gap-4">
        <label className="block font-medium" htmlFor="customRunName">
          Custom Run Name
        </label>
        <input
          id="customRunName"
          name="customRunName"
          type="text"
          value={form.customRunName}
          onChange={handleChange}
          required
          className="w-full rounded border-gray-300 shadow-sm focus:border-primary focus:ring-1 focus:ring-primary"
        />

        <label className="block font-medium" htmlFor="project">
          Project
        </label>
        <select
          id="project"
          name="project"
          value={form.project}
          onChange={handleChange}
          required
          className="w-full rounded border-gray-300 shadow-sm focus:border-primary focus:ring-1 focus:ring-primary"
        >
          <option value="" disabled>
            Select a project
          </option>
          {projects.map(p => (
            <option key={p} value={p}>
              {p}
            </option>
          ))}
        </select>

        <div className="flex justify-end space-x-3 mt-4">
          <button
            type="button"
            className="px-4 py-2 rounded bg-secondary text-white hover:bg-secondary-dark"
          >
            Cancel
          </button>
          <button
            type="submit"
            className="px-4 py-2 rounded bg-primary text-white hover:bg-primary-dark"
          >
            Submit
          </button>
        </div>
      </form>
    </Card>
  );
}