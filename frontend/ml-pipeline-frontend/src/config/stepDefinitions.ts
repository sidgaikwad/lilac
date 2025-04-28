import { StepDefinition } from '@/types';

// Hardcoded example step definitions for frontend development
// Replace this with API call to GET /step_definitions when backend is ready

export const hardcodedStepDefinitions: StepDefinition[] = [
  // --- Input ---
  {
    id: 'sd-input-s3', // Use 'id' to match frontend type
    type: 'S3Input', // Hypothetical type
    label: 'S3 Input',
    description: 'Loads images from an S3 bucket path.',
    category: 'Input',
    parameters: [
      {
        name: 'bucket',
        label: 'S3 Bucket',
        type: 'string',
        required: true,
        description: 'Name of the S3 bucket.',
      },
      {
        name: 'prefix',
        label: 'S3 Prefix/Path',
        type: 'string',
        required: true,
        description: 'Path within the bucket (e.g., images/raw/).',
      },
      {
        name: 'file_pattern',
        label: 'File Pattern',
        type: 'string',
        required: false,
        defaultValue: '*.jpg',
        description: 'Glob pattern to match files (e.g., *.png, **/*.jpeg).',
      },
    ],
  },
  // --- Processing ---
  {
    id: 'sd-blur-detector', // Use 'id'
    type: 'BlurDetector', // Matches backend StepType
    label: 'Blur Detector',
    description: 'Detects and potentially filters blurry images.',
    category: 'Processing',
    parameters: [
      {
        name: 'threshold',
        label: 'Blur Threshold',
        type: 'number',
        required: true,
        defaultValue: 100,
        description:
          'Laplacian variance threshold. Lower values detect less blur.',
      },
      {
        name: 'filter_blurry',
        label: 'Filter Blurry Images',
        type: 'boolean',
        required: false,
        defaultValue: true,
        description: 'If true, removes blurry images from the batch.',
      },
    ],
  },
  {
    id: 'sd-resolution-std', // Use 'id'
    type: 'ResolutionStandardizer', // Matches backend StepType
    label: 'Resize Images',
    description: 'Resizes images to a standard resolution.',
    category: 'Processing',
    parameters: [
      {
        name: 'width',
        label: 'Target Width',
        type: 'number',
        required: true,
        defaultValue: 512,
        description: 'Target width in pixels.',
      },
      {
        name: 'height',
        label: 'Target Height',
        type: 'number',
        required: true,
        defaultValue: 512,
        description: 'Target height in pixels.',
      },
      {
        name: 'filter_type',
        label: 'Resizing Filter',
        type: 'enum',
        required: false,
        defaultValue: 'Lanczos3',
        options: ['Nearest', 'Triangle', 'CatmullRom', 'Gaussian', 'Lanczos3'],
        description: 'Algorithm used for resizing.',
      },
    ],
  },
  // --- Output ---
  {
    id: 'sd-output-s3', // Use 'id'
    type: 'S3Output', // Hypothetical type
    label: 'S3 Output',
    description: 'Saves processed images to an S3 bucket path.',
    category: 'Output',
    parameters: [
      {
        name: 'bucket',
        label: 'S3 Bucket',
        type: 'string',
        required: true,
        description: 'Name of the S3 bucket.',
      },
      {
        name: 'prefix',
        label: 'S3 Prefix/Path',
        type: 'string',
        required: true,
        description:
          'Path within the bucket to save images (e.g., images/processed/).',
      },
      {
        name: 'format',
        label: 'Output Format',
        type: 'enum',
        required: false,
        defaultValue: 'jpg',
        options: ['jpg', 'png', 'webp'],
        description: 'Image format for saving.',
      },
    ],
  },
  // --- Example NoOp ---
  {
    id: 'sd-noop', // Use 'id'
    type: 'NoOp', // Matches backend StepType
    label: 'No Operation',
    description:
      'Passes data through without modification (for testing/debugging).',
    category: 'Utility',
    parameters: [], // No parameters
  },
];
