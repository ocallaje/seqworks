import type { PipelineDefinition } from './types'

const genomes = [
  { value: 'GRCh38', label: 'Human — GRCh38' },
  { value: 'GRCm39', label: 'Mouse — GRCm39' },
  { value: 'nhp', label: 'NHP — ' },
]

export const gencodeOptionsByGenome: Record<string, Option[]> = {
  GRCh38: [
    { value: 'v41', label: 'v41' },
    { value: 'v42', label: 'v42' },
    { value: 'v43', label: 'v43' },
    { value: 'v50', label: 'v50' },
  ],
  GRCm39: [
    { value: 'vM30', label: 'vM30' },
    { value: 'vM31', label: 'vM31' },
    { value: 'vM32', label: 'vM32' },
  ],
  nhp: [
    { value: 'v1', label: 'v1' },
  ],
}

const resources = {
  title: 'Compute resources',
  description: 'Upper limits applied to every task in the workflow.',
  fields: [
    { type: 'number', key: 'max_cpus', label: 'Max CPUs per task', default: 16, min: 1, max: 128, step: 1 },
    {
      type: 'select',
      key: 'max_memory',
      label: 'Max memory per task',
      default: '64.GB',
      options: [
        { value: '32.GB', label: '32 GB' },
        { value: '64.GB', label: '64 GB' },
        { value: '128.GB', label: '128 GB' },
        { value: '256.GB', label: '256 GB' },
      ],
    },
    {
      type: 'segmented',
      key: 'priority',
      label: 'Queue priority',
      default: 'normal',
      options: [
        { value: 'low', label: 'Low' },
        { value: 'normal', label: 'Normal' },
        { value: 'high', label: 'High' },
      ],
    },
  ],
} satisfies PipelineDefinition['sections'][number]

export const pipelines: PipelineDefinition[] = [
  {
    id: 'bulk-rnaseq',
    name: 'Bulk RNA-seq',
    workflow: 'thomsonbc/NF-RNAseq',
    version: 'latest',
    description: 'Quality control, trimming, alignment and gene-level quantification for bulk RNA-seq libraries.',
    steps: ['Trim', 'Align', 'Index', 'Quantify', 'MultiQC', 'DESeq2'],
    sections: [
      {
        title: 'Setup',
        fields: [
          {
            type: 'select',
            key: 'input',
            label: 'Project Folder',
            options: [
              { value: 'project-a', label: 'Project A' },
              { value: 'project-b', label: 'Project B' },
              { value: 'project-c', label: 'Project C' },
            ],
            default: 'project-a',
            help: 'Select the data folder',
            required: true,
          },
          {
            type: 'select',
            key: 'workflow',
            label: 'Workflow',
            options: [
              { value: 'default', label: 'default' },
              { value: 'qc_only', label: 'qc_only' },
              { value: 'de_analysis_only', label: 'de_analysis_only' },
            ],
            default: 'default',
            required: true,
            help: 'Choose a workflow for the pipeline'
          },
        ],
      },
      {
        title: 'Reference',
        fields: [
          { type: 'select', key: 'genome', label: 'Genome', default: 'GRCh38', options: genomes },
          {
            type: 'select',
            key: 'gencode',
            label: 'Gencode Release',
            default: 'gencode_45',
            options: [
              { value: 'gencode_45', label: 'GENCODE 45' },
              { value: 'gencode_44', label: 'GENCODE 44' },
              { value: 'ensembl_111', label: 'Ensembl 111' },
              { value: 'refseq', label: 'RefSeq' },
            ],
          },
        ],
      },
      {
        title: 'Alignment & quantification',
        fields: [
          {
            type: 'segmented',
            key: 'aligner',
            label: 'Aligner',
            default: 'star_salmon',
            options: [
              { value: 'star_salmon', label: 'STAR + Salmon' },
              { value: 'star_rsem', label: 'STAR + RSEM' },
              { value: 'hisat2', label: 'HISAT2' },
            ],
          },
          {
            type: 'segmented',
            key: 'strandedness',
            label: 'Library strandedness',
            default: 'auto',
            options: [
              { value: 'auto', label: 'Auto-detect' },
              { value: 'forward', label: 'Forward' },
              { value: 'reverse', label: 'Reverse' },
              { value: 'unstranded', label: 'Unstranded' },
            ],
          },
          {
            type: 'toggle',
            key: 'pseudo_aligner',
            label: 'Run Salmon pseudo-alignment',
            help: 'Adds transcript-level quantification alongside the main aligner.',
            default: true,
          },
        ],
      },
      {
        title: 'Trimming',
        fields: [
          {
            type: 'segmented',
            key: 'trimmer',
            label: 'Trimmer',
            default: 'trimgalore',
            options: [
              { value: 'trimgalore', label: 'Trim Galore' },
              { value: 'fastp', label: 'fastp' },
            ],
          },
          { type: 'number', key: 'min_trimmed_reads', label: 'Min reads after trimming', default: 10000, min: 0, step: 1000 },
        ],
      },
      {
        title: 'Quality control',
        fields: [
          {
            type: 'multi',
            key: 'qc_tools',
            label: 'QC modules',
            default: ['fastqc', 'rseqc', 'qualimap'],
            options: [
              { value: 'fastqc', label: 'FastQC' },
              { value: 'rseqc', label: 'RSeQC' },
              { value: 'qualimap', label: 'Qualimap' },
              { value: 'dupradar', label: 'dupRadar' },
              { value: 'preseq', label: 'Preseq' },
              { value: 'deseq2_qc', label: 'DESeq2 QC' },
            ],
          },
        ],
      },
      resources,
    ],
  },
  {
    id: 'scrnaseq',
    name: 'Single-cell RNA-seq',
    workflow: 'ocallaje/NF-scRNAseq',
    version: 'multi-sample',
    description: 'Droplet-based single-cell quantification, cell calling and filtering.',
    steps: ['Demultiplex', 'Index', 'Align', 'Cell calling', 'Filter', 'MultiQC'],
    sections: [
      {
        title: 'Input & output',
        fields: [
          { type: 'text', key: 'input', label: 'Samplesheet', placeholder: 's3://genelink-data/projects/sc11/samplesheet.csv', required: true, mono: true },
          { type: 'text', key: 'outdir', label: 'Output directory', placeholder: 's3://genelink-results/sc11/', required: true, mono: true },
        ],
      },
      {
        title: 'Chemistry & reference',
        fields: [
          {
            type: 'select',
            key: 'protocol',
            label: 'Chemistry',
            default: '10XV3',
            options: [
              { value: '10XV2', label: "10x Chromium 3' v2" },
              { value: '10XV3', label: "10x Chromium 3' v3 / v3.1" },
              { value: '10XV4', label: "10x Chromium 3' v4" },
              { value: '10X5', label: "10x Chromium 5'" },
            ],
          },
          { type: 'select', key: 'genome', label: 'Genome', default: 'GRCh38', options: genomes },
          {
            type: 'segmented',
            key: 'aligner',
            label: 'Quantifier',
            default: 'star',
            options: [
              { value: 'star', label: 'STARsolo' },
              { value: 'alevin', label: 'Alevin-fry' },
              { value: 'kallisto', label: 'kallisto | bustools' },
              { value: 'cellranger', label: 'Cell Ranger' },
            ],
          },
        ],
      },
      {
        title: 'Cell filtering',
        fields: [
          { type: 'number', key: 'expected_cells', label: 'Expected cells per sample', default: 5000, min: 100, step: 500 },
          { type: 'number', key: 'min_genes', label: 'Min genes per cell', default: 200, min: 0, step: 50 },
          { type: 'number', key: 'max_mito', label: 'Max mitochondrial reads', default: 15, min: 0, max: 100, step: 1, unit: '%' },
          { type: 'toggle', key: 'remove_doublets', label: 'Remove doublets (scDblFinder)', default: true },
        ],
      },
      resources,
    ],
  },
  {
    id: 'fusion-detection',
    name: 'Fusion Detection',
    workflow: 'nf-core/rnafusion',
    version: '3.0.2',
    description: 'Detect gene fusions with an ensemble of callers and a consolidated clinical-style report.',
    steps: ['Trim', 'Align', 'STAR-Fusion', 'Arriba', 'Merge', 'Report'],
    sections: [
      {
        title: 'Input & output',
        fields: [
          { type: 'text', key: 'input', label: 'Samplesheet', placeholder: 's3://genelink-data/projects/onc7/samplesheet.csv', required: true, mono: true },
          { type: 'text', key: 'outdir', label: 'Output directory', placeholder: 's3://genelink-results/onc7/', required: true, mono: true },
        ],
      },
      {
        title: 'Callers',
        fields: [
          {
            type: 'multi',
            key: 'tools',
            label: 'Fusion callers',
            default: ['starfusion', 'arriba'],
            options: [
              { value: 'starfusion', label: 'STAR-Fusion' },
              { value: 'arriba', label: 'Arriba' },
              { value: 'fusioncatcher', label: 'FusionCatcher' },
              { value: 'ctatsplicing', label: 'CTAT-Splicing' },
            ],
          },
          { type: 'select', key: 'genome', label: 'Genome', default: 'GRCh38', options: genomes.slice(0, 2) },
          { type: 'number', key: 'min_reads', label: 'Min supporting reads', default: 3, min: 1, max: 50, step: 1 },
          { type: 'toggle', key: 'fusion_report', label: 'Generate fusion report', default: true },
        ],
      },
      resources,
    ],
  },
  {
    id: 'read-qc',
    name: 'Read QC & Trimming',
    workflow: 'genelink/readqc',
    version: '0.9.0',
    description: 'Fast pre-flight QC, adapter trimming and contamination screening before full analysis.',
    steps: ['FastQC', 'fastp', 'FastQ Screen', 'MultiQC'],
    sections: [
      {
        title: 'Input & output',
        fields: [
          { type: 'text', key: 'input', label: 'FASTQ directory or samplesheet', placeholder: 's3://genelink-data/runs/240611_NB552/', required: true, mono: true },
          { type: 'text', key: 'outdir', label: 'Output directory', placeholder: 's3://genelink-results/qc/240611/', required: true, mono: true },
        ],
      },
      {
        title: 'Options',
        fields: [
          {
            type: 'segmented',
            key: 'read_type',
            label: 'Read layout',
            default: 'paired',
            options: [
              { value: 'paired', label: 'Paired-end' },
              { value: 'single', label: 'Single-end' },
            ],
          },
          { type: 'number', key: 'min_length', label: 'Min read length', default: 36, min: 1, max: 300, step: 1, unit: 'bp' },
          { type: 'number', key: 'quality', label: 'Quality cutoff', default: 20, min: 0, max: 40, step: 1, unit: 'Phred' },
          { type: 'toggle', key: 'screen', label: 'Contamination screen (FastQ Screen)', default: true },
        ],
      },
    ],
  },
]

export function getPipeline(id: string | undefined) {
  return pipelines.find((p) => p.id === id)
}
