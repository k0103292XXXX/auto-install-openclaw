# auto-install-openclaw — 프로젝트 스펙

## 개요
OpenClaw를 일반 사용자도 쉽게 설치할 수 있는 GUI 기반 원클릭 스텁 인스톨러.
Windows용 Tauri 앱으로 제작, 오픈소스(MIT) 공개.

## 핵심 철학
- **Next → Next → Finish** — 일반인도 설치 가능
- **스텁 인스톨러** — 작은 exe(~3MB), 실제 컴포넌트는 온라인에서 다운로드
- **자동화 우선** — 사용자 입력 최소화

## 설치 플로우

```
1. Setup.exe 실행 (~3MB)
2. 환영 화면 → "시작하기" 클릭
3. "GitHub로 로그인" → 브라우저 OAuth 팝업
4. [자동] GitHub Copilot 구독 상태 확인
   ├─ 구독 O → 다음 단계
   └─ 구독 X → 안내 메시지 + 가입 링크 + "구독 후 재시도" 버튼
5. 설치 경로 선택 (기본값 제공)
6. "설치" 클릭
7. [자동] 백그라운드 설치:
   a. Node.js 설치 여부 확인 → 없으면 공식 LTS 다운로드 & 설치
   b. npm install -g openclaw
   c. openclaw.yaml 생성 (GitHub Copilot Claude Opus 4 자동 등록)
   d. openclaw gateway start
   e. 시스템 시작프로그램 등록 (선택)
8. 설치 완료 화면
   └─ "텔레그램 연결하기" / "웹챗 열기" 버튼
```

## 기술 스택

| 항목 | 선택 | 이유 |
|------|------|------|
| GUI 프레임워크 | Tauri v2 | 경량(~3MB), Rust 백엔드 |
| 프론트엔드 | Vanilla JS + HTML/CSS | 의존성 최소화 |
| OAuth | GitHub Device Flow | CLI/데스크톱 친화적 |
| Node.js 설치 | 공식 .msi 사일런트 설치 | `node-v{ver}-x64.msi /quiet` |
| 빌드 | tauri-cli + GitHub Actions | 자동 빌드/릴리즈 |

## GitHub OAuth 플로우 (Device Flow)

> Device Flow는 브라우저 리다이렉트 없이도 동작하므로 데스크톱 앱에 적합.

```
1. POST https://github.com/login/device/code
   → device_code, user_code, verification_uri 받음
2. 사용자에게 verification_uri 표시 + user_code 복사
   (또는 브라우저 자동 오픈)
3. 사용자가 브라우저에서 코드 입력 & 승인
4. 앱이 polling으로 access_token 획득
   POST https://github.com/login/oauth/access_token
5. access_token으로 Copilot 구독 상태 확인
```

## Copilot 구독 확인

```
GET https://api.github.com/copilot_billing/subscriptions
Authorization: Bearer {token}

→ 구독 있음: 200 + subscription data
→ 구독 없음: 404 또는 빈 응답
```

> ⚠️ 정확한 API 엔드포인트는 개발 시 재확인 필요 (GitHub API 변경 가능)

## openclaw.yaml 자동 생성

```yaml
llm:
  providers:
    - id: github-copilot
      type: copilot
      models:
        - claude-opus-4
      default: true
      auth:
        token: "{oauth_token}"

gateway:
  autostart: true
```

> 실제 openclaw 설정 구조에 맞게 조정 필요

## 파일 구조

```
auto-install-openclaw/
├── src-tauri/          # Rust 백엔드
│   ├── src/
│   │   ├── main.rs     # Tauri 엔트리포인트
│   │   ├── installer.rs # 설치 로직
│   │   ├── github.rs   # OAuth + Copilot 체크
│   │   └── node.rs     # Node.js 설치 관리
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                # 프론트엔드 (웹)
│   ├── index.html      # 메인 UI
│   ├── style.css
│   └── main.js
├── .github/
│   └── workflows/
│       └── release.yml # 자동 빌드/릴리즈
├── SPEC.md             # 이 문서
├── README.md
├── README.ko.md
├── LICENSE             # MIT
└── .gitignore
```

## 미래 확장

- [ ] macOS 지원
- [ ] Linux 지원
- [ ] 텔레그램 봇 자동 생성 (BotFather API)
- [ ] 업데이트 체커 (새 버전 알림)
- [ ] 다국어 UI

## 라이선스

MIT License
