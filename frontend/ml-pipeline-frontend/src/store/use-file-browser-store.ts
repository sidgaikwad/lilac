import { createWithEqualityFn } from 'zustand/traditional';

interface Folder {
  prefix: string;
  level: number;
  name: string;
}

export type FileBrowserState = {
  currentPath: string[];
  folders: { [key: string]: Folder };
  onFolderBack: () => void;
  onFolderEnter: (prefix: string) => void;
  registerFolder: (prefix: string, level?: number) => void;
  getFolder: (prefix: string) => Folder;
};

const useFileBrowserStore = createWithEqualityFn<FileBrowserState>(
  (set, get) => ({
    currentPath: [],
    folders: {},
    onFolderBack: () => {
      const path = get().currentPath;
      if (path.length > 1) {
        path.pop();
        set({ currentPath: path });
      }
    },
    onFolderEnter: (prefix: string) => {
      const folder = get().folders[prefix];
      const path = get().currentPath.slice(0, folder.level);
      path.push(prefix);
      set({ currentPath: path });
    },
    registerFolder: (prefix: string, level: number = 0) => {
      const folders = get().folders;

      const name = prefix.split('/').at(-2);
      const folder = {
        prefix,
        level,
        name: name ?? prefix,
      };
      folders[folder.prefix] = folder;
      set({ folders: folders });
    },
    getFolder: (prefix: string) => {
      return get().folders[prefix];
    },
  })
);

export default useFileBrowserStore;
