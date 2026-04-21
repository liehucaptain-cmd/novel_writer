export interface CharRelationship {
  target_id: string
  type: '朋友' | '敌人' | '亲人' | '恋人' | '师生' | '竞争' | '未知'
}

export interface Character {
  id: string
  name: string
  description: string
  age: string
  appearance: string
  personality: string
  background: string
  status: '待出场' | '已出场' | '重要' | '已下线'
  group: string
  appear_chapters: string[]
  relationships: CharRelationship[]
}

export interface Plotline {
  id: string
  name: string
  color: string
  status: 'active' | 'completed'
  description: string
  status_summary: string
  chapter_ids: string[]
}

export interface Chapter {
  id: string
  title: string
  content: string
  order: number
  characters_appeared: string[]
  plotline_ids: string[]
}

export interface Book {
  id: string
  name: string
  created_at: string
  updated_at: string
  chapters: Chapter[]
  characters: Character[]
  storylines: Plotline[]
}

export interface BookMeta {
  id: string
  name: string
  created_at: string
  updated_at: string
}

export interface BookIndex {
  books: BookMeta[]
}

export interface Scene {
  id: string
  name: string
  status: '待写' | '进行中' | '已完成'
  content: string
}

export interface OutlineChapter {
  id: string
  name: string
  scenes: Scene[]
}

export interface Act {
  id: string
  name: string
  color: string
  chapters: OutlineChapter[]
}

export interface Outline {
  acts: Act[]
}
