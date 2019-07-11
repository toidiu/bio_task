export interface TaskResp {
  tasks: Array<Task>;
}

export interface Task {
  itemId: number;
  projectId: number;
  title: string,
  description: string;
}

