import { useState, useEffect, useCallback } from 'react';
import { useParams } from 'react-router-dom';
import { getProjectQuery } from '@/services';
import useProjectStore from '@/store/use-project-store';
import {
  Container,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
} from '@/components/ui/container';
import Breadcrumbs from '@/components/common/breadcrumbs';
import { useSuspenseQuery } from '@tanstack/react-query';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { Button } from '@/components/ui/button';

const BASE_DEFAULT_MARKDOWN_CONTENT = `
## Getting Started: Your First Steps

Ready to get started? Here’s how to kick things off:

1.  **Upload Your Data**: The journey begins with your data. Navigate to the **Datasets** section (you'll find it in the sidebar to your left) to upload your first dataset. This will be the foundation for your project.

2.  **Build Your Workflow**: Once your data is in, you can begin to process, transform, and analyze your dataset.

Let's build something great!

---

## Project Notes & Ideas
*Use this space to jot down any thoughts, to-do items, or important information about this project.*

- 
- 
- 
`;

type ProjectParams = {
  projectId: string;
};

function ProjectDashboardPage() {
  const { projectId } = useParams<ProjectParams>();
  const { data: project } = useSuspenseQuery(getProjectQuery(projectId));
  const { setSelectedProjectId } = useProjectStore();

  const getProjectSpecificDefaultContent = useCallback(() => {
    return BASE_DEFAULT_MARKDOWN_CONTENT.replace(
      '{{projectName}}',
      project?.name || 'this project'
    );
  }, [project?.name]);

  const [markdownContent, setMarkdownContent] = useState(
    getProjectSpecificDefaultContent()
  );
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => {
    setSelectedProjectId(project?.id);
  }, [setSelectedProjectId, project?.id]);

  useEffect(() => {
    if (projectId) {
      const savedContent = localStorage.getItem(
        `projectDashboard_${projectId}`
      );
      if (savedContent) {
        setMarkdownContent(savedContent);
      } else {
        setMarkdownContent(getProjectSpecificDefaultContent());
      }
    }
  }, [projectId, getProjectSpecificDefaultContent]);

  useEffect(() => {
    if (projectId) {
      localStorage.setItem(`projectDashboard_${projectId}`, markdownContent);
    }
  }, [projectId, markdownContent]);

  return (
    <Container>
      <ContainerHeader>
        <div className='flex-1 shrink-0 grow-0 basis-full pb-4'>
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Projects',
                link: `/`,
              },
              {
                content: project.name,
                link: `/projects/${projectId}`,
              },
            ]}
          />
        </div>
      </ContainerHeader>

      <ContainerContent>
        <div className='mb-6 flex items-center justify-between border-b pb-4'>
          <h1 className='text-3xl font-bold'>
            <span className='text-primary'>
              {project?.name || 'Your Project'}
            </span>
          </h1>
          <img
            src='/logo.png'
            alt='Application Logo'
            style={{ width: '100px', height: 'auto' }}
          />
        </div>
        <ContainerDescription></ContainerDescription>

        <div className='mb-4 flex justify-end'>
          <Button
            variant='outline'
            size='sm'
            onClick={() => setIsEditing(!isEditing)}
          >
            {isEditing ? 'View Content' : 'Edit Content'}
          </Button>
        </div>

        {isEditing ? (
          <textarea
            value={markdownContent}
            onChange={(e) => setMarkdownContent(e.target.value)}
            className='focus:ring-primary focus:border-primary h-96 w-full rounded-md border p-2 shadow-sm'
            placeholder='Enter your project notes in Markdown...'
          />
        ) : (
          <div className='prose dark:prose-invert lg:prose-xl bg-card text-card-foreground min-h-[24rem] max-w-none rounded-md border p-2'>
            <ReactMarkdown remarkPlugins={[remarkGfm]}>
              {markdownContent ||
                "*No content yet. Click 'Edit Content' to add your notes.*"}
            </ReactMarkdown>
          </div>
        )}
      </ContainerContent>
    </Container>
  );
}
export default ProjectDashboardPage;
