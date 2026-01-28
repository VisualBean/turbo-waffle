export type ConnectionType = 'website' | 'ssh'
export type HealthStatus = 'online' | 'offline' | 'degraded' | 'unknown'

export interface WebsiteConfig {
  type: 'website'
  url: string
  checkPath?: string
}

export interface SshConfig {
  type: 'ssh'
  host: string
  port: number
  username: string
  wolEnabled: boolean
  macAddress?: string
  broadcastAddr?: string
}

export type ConnectionConfig = WebsiteConfig | SshConfig

export interface Connection {
  id: string
  name: string
  icon?: string
  iconColor?: string
  order: number
  config: ConnectionConfig
  createdAt: string
  updatedAt: string
}

export const BRAND_ICONS = [
  { name: 'Default', class: '' },
  { name: 'GitHub', class: 'fa-brands fa-github' },
  { name: 'GitLab', class: 'fa-brands fa-gitlab' },
  { name: 'Gitea', class: 'fa-brands fa-git-alt' },
  { name: 'Bitbucket', class: 'fa-brands fa-bitbucket' },
  { name: 'Docker', class: 'fa-brands fa-docker' },
  { name: 'Kubernetes', class: 'fa-solid fa-dharmachakra' },
  { name: 'AWS', class: 'fa-brands fa-aws' },
  { name: 'Google Cloud', class: 'fa-brands fa-google' },
  { name: 'Azure', class: 'fa-brands fa-microsoft' },
  { name: 'DigitalOcean', class: 'fa-brands fa-digital-ocean' },
  { name: 'Linode', class: 'fa-brands fa-linode' },
  { name: 'Linux', class: 'fa-brands fa-linux' },
  { name: 'Ubuntu', class: 'fa-brands fa-ubuntu' },
  { name: 'Debian', class: 'fa-brands fa-debian' },
  { name: 'RedHat', class: 'fa-brands fa-redhat' },
  { name: 'CentOS', class: 'fa-brands fa-centos' },
  { name: 'Fedora', class: 'fa-brands fa-fedora' },
  { name: 'FreeBSD', class: 'fa-brands fa-freebsd' },
  { name: 'Windows', class: 'fa-brands fa-windows' },
  { name: 'Apple', class: 'fa-brands fa-apple' },
  { name: 'Raspberry Pi', class: 'fa-brands fa-raspberry-pi' },
  { name: 'Proxmox', class: 'fa-solid fa-cubes' },
  { name: 'VMware', class: 'fa-solid fa-layer-group' },
  { name: 'NAS', class: 'fa-solid fa-hard-drive' },
  { name: 'Plex', class: 'fa-solid fa-play' },
  { name: 'Jellyfin', class: 'fa-solid fa-film' },
  { name: 'Home Assistant', class: 'fa-solid fa-house-signal' },
  { name: 'Nextcloud', class: 'fa-solid fa-cloud' },
  { name: 'Grafana', class: 'fa-solid fa-chart-line' },
  { name: 'Prometheus', class: 'fa-solid fa-fire' },
  { name: 'Pi-hole', class: 'fa-solid fa-shield-halved' },
  { name: 'Nginx', class: 'fa-solid fa-n' },
  { name: 'Apache', class: 'fa-solid fa-feather' },
  { name: 'Traefik', class: 'fa-solid fa-road' },
  { name: 'Caddy', class: 'fa-solid fa-lock' },
  { name: 'Redis', class: 'fa-solid fa-bolt' },
  { name: 'MongoDB', class: 'fa-solid fa-leaf' },
  { name: 'Jenkins', class: 'fa-brands fa-jenkins' },
  { name: 'Terraform', class: 'fa-solid fa-cube' },
  { name: 'Vault', class: 'fa-solid fa-vault' },
  { name: 'Slack', class: 'fa-brands fa-slack' },
  { name: 'Discord', class: 'fa-brands fa-discord' },
  { name: 'Wordpress', class: 'fa-brands fa-wordpress' },
  { name: 'Drupal', class: 'fa-brands fa-drupal' },
  { name: 'Joomla', class: 'fa-brands fa-joomla' },
  { name: 'Cloudflare', class: 'fa-brands fa-cloudflare' },
  { name: 'Database', class: 'fa-solid fa-database' },
  { name: 'Server', class: 'fa-solid fa-server' },
  { name: 'Network', class: 'fa-solid fa-network-wired' },
  { name: 'Router', class: 'fa-solid fa-wifi' },
  { name: 'Firewall', class: 'fa-solid fa-fire-flame-curved' },
  { name: 'VPN', class: 'fa-solid fa-user-shield' },
  { name: 'House', class: 'fa-solid fa-house' },
  { name: 'Globe', class: 'fa-solid fa-globe' },
  { name: 'Terminal', class: 'fa-solid fa-terminal' },
  { name: 'Code', class: 'fa-solid fa-code' },
  { name: 'Gamepad', class: 'fa-solid fa-gamepad' },
  { name: 'Camera', class: 'fa-solid fa-video' },
  { name: 'Music', class: 'fa-solid fa-music' },
  { name: 'Book', class: 'fa-solid fa-book' },
  { name: 'Download', class: 'fa-solid fa-download' },
  { name: 'Folder', class: 'fa-solid fa-folder' },
] as const

export const ICON_COLORS = [
  '#ffffff',
  '#000000',
  '#3b82f6',
  '#06b6d4',
  '#14b8a6',
  '#22c55e',
  '#84cc16',
  '#eab308',
  '#f97316',
  '#ef4444',
  '#ec4899',
  '#a855f7',
  '#6366f1',
  '#64748b',
] as const

export interface HealthResult {
  connectionId: string
  status: HealthStatus
  latencyMs?: number
  error?: string
  checkedAt: string
}
