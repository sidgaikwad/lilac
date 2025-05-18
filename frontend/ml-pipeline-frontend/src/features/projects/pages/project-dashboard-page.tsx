import React, { useState, useEffect, useCallback } from 'react';
import useOrganizationStore from '@/store/use-organization-store';
import { useParams } from 'react-router-dom';
import { getProjectQuery } from '@/services';
import {
  Container,
  ContainerAction,
  ContainerContent,
  ContainerDescription,
  ContainerHeader,
  ContainerTitle,
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

2.  **Build Your Workflow**: Once your data is in, head over to the **Pipelines** section. Here, you can design and create a pipeline – a series of steps to process, transform, and analyze your dataset.

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
  const { setSelectedProjectId, setSelectedOrganizationId } =
    useOrganizationStore();

  const getProjectSpecificDefaultContent = useCallback(() => {
    return BASE_DEFAULT_MARKDOWN_CONTENT.replace('{{projectName}}', project?.name || 'this project');
  }, [project?.name]);

  const [markdownContent, setMarkdownContent] = useState(getProjectSpecificDefaultContent());
  const [isEditing, setIsEditing] = useState(false);

  useEffect(() => {
    setSelectedOrganizationId(project?.organizationId);
    setSelectedProjectId(project?.id);
  }, [
    setSelectedOrganizationId,
    setSelectedProjectId,
    project?.id,
    project?.organizationId,
  ]);

  useEffect(() => {
    if (projectId) {
      const savedContent = localStorage.getItem(`projectDashboard_${projectId}`);
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
        <div className="flex-1 shrink-0 grow-0 basis-full pb-4">
          <Breadcrumbs
            breadcrumbs={[
              {
                content: 'Projects',
                link: `/organizations/${project?.organizationId}/projects`,
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
        {/* Static Welcome Header with Logo */}
        <div className="flex items-center justify-between mb-6 pb-4 border-b">
          <h1 className="text-3xl font-bold">
            <span className="text-primary">{project?.name || 'Your Project'}</span>
          </h1>
          <img src="/logo.png" alt="Application Logo" style={{ width: '100px', height: 'auto' }} />
        </div>
          <ContainerDescription>
            
          </ContainerDescription>

        <div className="flex justify-end mb-4">
          <Button variant="outline" size="sm" onClick={() => setIsEditing(!isEditing)}>
            {isEditing ? 'View Content' : 'Edit Content'}
          </Button>
        </div>

        {isEditing ? (
          <textarea
            value={markdownContent}
            onChange={(e) => setMarkdownContent(e.target.value)}
            className="w-full h-96 p-2 border rounded-md shadow-sm focus:ring-primary focus:border-primary"
            placeholder="Enter your project notes in Markdown..."
          />
        ) : (
          <div className="prose dark:prose-invert lg:prose-xl max-w-none p-2 border rounded-md bg-card text-card-foreground min-h-[24rem]">
            <ReactMarkdown 
              remarkPlugins={[remarkGfm]}
            >
              {markdownContent || "*No content yet. Click 'Edit Content' to add your notes.*"}
            </ReactMarkdown>
          </div>
        )}
      </ContainerContent>
    </Container>
  );
}
export default ProjectDashboardPage;
